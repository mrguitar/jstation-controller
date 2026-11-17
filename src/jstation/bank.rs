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
/// Only `OneProgramResp` procedures are collected. Real `.syx` bank/patch
/// backups produced by the original J-Edit application (verified against
/// several real-world sample files) were found to lead with one extra SysEx
/// message using an undocumented procedure id (`0x70`) that this crate has
/// no definition for. So, rather than relying on the strict, single-pass
/// `sysex::parse` (which aborts the whole buffer on the first procedure it
/// doesn't recognize), each complete `F0`..`F7` message is located and
/// parsed independently, and silently skipped if it isn't a `OneProgramResp`
/// we can decode.
pub fn parse(mut input: &[u8]) -> Result<Vec<Program>, Error> {
    let mut programs = Vec::new();

    while let Some(start) = input.iter().position(|&byte| byte == midi::sysex::TAG) {
        input = &input[start..];

        let Some(end) = input.iter().position(|&byte| byte == midi::sysex::END_TAG) else {
            // Truncated trailing message: nothing more we can parse.
            break;
        };

        let (msg, rest) = input.split_at(end + 1);
        input = rest;

        match sysex::parse(msg) {
            Ok((_, msg)) => {
                if let Procedure::OneProgramResp(resp) = msg.proc {
                    programs.push(resp.prog);
                }
            }
            Err(err) => log::debug!("Bank file: skipping unrecognized message: {err:?}"),
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
