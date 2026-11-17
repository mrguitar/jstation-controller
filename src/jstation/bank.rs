use crate::jstation::{
    procedure::OneProgramResp, sysex, Error, Procedure, ProcedureBuilder, Program,
};
use crate::midi;

/// Channel used to frame Programs when serializing to / parsing from a bank file.
///
/// The channel byte only matters for a live MIDI connection, so an arbitrary,
/// device agnostic value is used when writing to, or reading from, a file.
const FILE_CHANNEL: midi::Channel = midi::Channel::ALL;

/// Serializes the provided `Program`s to a J-Station bank file buffer.
///
/// The resulting bytes are a bare concatenation of complete `OneProgramResp`
/// SysEx messages, one per `Program`, using the very same framing as a live
/// Program dump from the device.
///
/// Note: a live Bank Dump from the device brackets those messages with
/// `StartBankDumpResp` / `EndBankDumpResp` markers, but those only carry a
/// byte count that isn't needed to parse this file back (see `parse`
/// below), so they are omitted here to keep the format as simple as
/// possible.
pub fn serialize<'a>(programs: impl Iterator<Item = &'a Program>) -> Vec<u8> {
    let mut buf = Vec::new();

    for prog in programs {
        buf.extend(OneProgramResp::from(prog).build_for(FILE_CHANNEL));
    }

    buf
}

/// Parses the `Program`s found in a J-Station bank file buffer.
///
/// Only `OneProgramResp` procedures are collected: the Start / End Bank Dump
/// markers, if present, are ignored. This also allows parsing a file
/// produced by an actual live Bank Dump capture, markers included.
pub fn parse(mut input: &[u8]) -> Result<Vec<Program>, Error> {
    let mut programs = Vec::new();

    while !input.is_empty() {
        let (rest, msg) = sysex::parse(input).map_err(|err| {
            log::error!("Bank file: {err}");

            Error::Parse
        })?;

        input = rest;

        if let Procedure::OneProgramResp(resp) = msg.proc {
            programs.push(resp.prog);
        }
    }

    if programs.is_empty() {
        return Err(Error::EmptyBankFile);
    }

    Ok(programs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jstation::{data::ProgramData, split_bytes, ProgramId, ProgramNb};

    fn test_program(nb: u8, name: &str) -> Program {
        let mut checksum = 0u8;
        let mut buf = Vec::new();

        for _ in 0..ProgramData::PARAM_COUNT {
            buf.extend(split_bytes::from_u8(0));
        }

        for byte in name.as_bytes().iter().chain(std::iter::once(&0u8)) {
            buf.extend(split_bytes::from_u8(*byte));
        }

        let len = (ProgramData::PARAM_COUNT + name.len() + 1) as u16;
        let (_, data) = ProgramData::parse(&buf, &mut checksum, len).unwrap();

        Program::new(ProgramId::new_user(ProgramNb::try_from(nb).unwrap()), data)
    }

    #[test]
    fn serialize_parse_round_trip() {
        let progs = [test_program(0, "One"), test_program(1, "Two")];

        let buf = serialize(progs.iter());
        let parsed = parse(&buf).expect("parse bank file");

        assert_eq!(parsed.len(), progs.len());

        for (orig, got) in progs.iter().zip(parsed.iter()) {
            assert_eq!(orig.id(), got.id());
            assert_eq!(orig.name(), got.name());
            assert_eq!(orig.data().buf(), got.data().buf());
        }
    }

    #[test]
    fn parse_empty_is_error() {
        assert!(matches!(parse(&[]), Err(Error::EmptyBankFile)));
    }

    #[test]
    fn single_program_round_trip() {
        let prog = test_program(0, "Solo");

        let buf = serialize(std::iter::once(&prog));
        let parsed = parse(&buf).expect("parse one program bank file");

        assert_eq!(parsed.len(), 1);
    }
}
