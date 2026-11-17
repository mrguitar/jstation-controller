# Direction 3 art manifest — jstation-controller cabinet/amp macro crops

All sources: Wikimedia Commons, CC-licensed or public domain. Style: amps get a
control-panel knob-row macro (300x100); cabinets/acoustics get a grille-cloth +
corner macro (160x160). Every crop avoids brand script/logos by framing, not
retouching, except where noted.

## Amp models (`src/jstation/data/dsp/amp.rs`)

| Nick | Real-world name | Status | Final file | Commons file title | Artist | License | Source URL | Notes |
|---|---|---|---|---|---|---|---|---|
| J Crunch | JM150 Millennium Crunch | skipped | — | — | — | — | — | Johnson's own discontinued amp; not found on Commons. |
| J Solo | JM150 Millennium Solo | skipped | — | — | — | — | — | Same as above. |
| J Clean | JM150 Millennium Clean | skipped | — | — | — | — | — | Same as above. |
| Boutique | Matchless DC30 | skipped | — | — | — | — | — | Only Commons hit (Matchless HC-30, a different model) was 227×101px — too small to crop. |
| Rectified | MesaBoogie Dual Rectifier | done (prior pass) | amp_mesa_dualrect.jpg | Dual rectifier Roadster Mesa Boogie Head.jpg | fvancini | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:Dual_rectifier_Roadster_Mesa_Boogie_Head.jpg | |
| Brit Stack | Marshall JCM900 | done (this pass) | amp_marshall_jcm900.jpg | Marshall JCM 900 Guitar Amp Setting - Simokitazawa MOSAIC, 2013-11-23 14.55.25 (by Junya Ogura).jpg | Junya Ogura from Tokyo, Japan | CC BY-SA 2.0 | https://commons.wikimedia.org/wiki/File:Marshall_JCM_900_Guitar_Amp_Setting_-_Simokitazawa_MOSAIC,_2013-11-23_14.55.25_(by_Junya_Ogura).jpg | Replaces the dim/cluttered JCM900 shot from the prior pass. |
| Brit Combo | '63 Vox AC30 top boost | done (prior pass) | amp_vox.jpg | VOX AC30 CC2 - panel-controls.jpg | massygo from Ghent, Belgium (edit by user:Retired electrician) | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:VOX_AC30_CC2_-_panel-controls.jpg | |
| Black Face | '65 Fender Twin Reverb | done (prior pass) | amp_fender.jpg | Fender Twin Reverb Blonde.jpg | Paulus 2 | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Fender_Twin_Reverb_Blonde.jpg | |
| Boat Back | piezo acoustic guitar | done (this pass) | amp_boatback_ovation.jpg | Nancy Wilson (Heart)'s 1978 Ovation Adamas 1687-7 (serial no. 477-92) specially made for her - Detail - Play It Loud. MET (2019-05-13 19.31.32 by Eden, Janine and Jim).jpg | Eden, Janine and Jim from New York City | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:Nancy_Wilson_(Heart)%27s_1978_Ovation_Adamas_1687-7_(serial_no._477-92)_specially_made_for_her_-_Detail_-_Play_It_Loud._MET_(2019-05-13_19.31.32_by_Eden,_Janine_and_Jim).jpg | **Concept switch**: cropped to the bridge/saddle area, not a knob row (piezo acoustic sim has no knobs). Source is a one-off custom Ovation Adamas with ornate inlay — representative of the "boat back" (Ovation roundback) idea, not a generic instrument. |
| Flat Top | dreadnaught acoustic guitar | done (this pass) | amp_flattop_hummingbird.jpg | 2005 Gibson Hummingbird - pickguard details with soundhole, bridge (2011-01-07 18.02.41 by John Tuggle).jpg | John Tuggle from Decatur, Ga, USA | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:2005_Gibson_Hummingbird_-_pickguard_details_with_soundhole,_bridge_(2011-01-07_18.02.41_by_John_Tuggle).jpg | **Concept switch**: soundhole/bridge/pickguard macro instead of a knob row. |
| Hot Rod | Mesa Boogie Mark II C | done (this pass) | amp_mesa_markiv.jpg | MarkIV.jpg | Original uploader and author was Fatty Pubis at en.wikipedia | Public domain | https://commons.wikimedia.org/wiki/File:MarkIV.jpg | **Substitution**: no Mark II C photo found; used a Mesa Boogie Mark IV (same family/era, very similar knob-row look). Shows "Boogie" badge below the knob row, cropped out. |
| Tweed | '57 Fender Tweed Deluxe | done (this pass, weak) | amp_tweed_deluxe.jpg | Fender1953Deluxe.jpg | Johnny Bgood123 | CC BY-SA 3.0 de | https://commons.wikimedia.org/wiki/File:Fender1953Deluxe.jpg | **Concept switch**: this amp's control panel is on the rear/top chassis edge, not visible from the front in any Commons photo found — fell back to a tweed-cloth + binding macro (cabinet-style) instead of a knob row, per the front-facing-only constraint confirmed this session. |
| Blues | dynamic blues tube combo (generic) | done (this pass) | amp_blues_superreverb.jpg | 1964 Fender Super Reverb.JPG | GabeMc | CC BY-SA 3.0 | https://commons.wikimedia.org/wiki/File:1964_Fender_Super_Reverb.JPG | **Generic stand-in**: nick names no real amp; used a blackface Fender Super Reverb as a representative "blues combo" knob row. |
| Fuzz | 60's fuzz tone | done (this pass) — **flag for review** | amp_fuzz_fuzzface.jpg | Dallas Arbiter Fuzz Face.jpg | sploshette | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:Dallas_Arbiter_Fuzz_Face.jpg | **Creative substitution**: this is a fuzz pedal, not an amp — cropped to the Volume knob + "FUZZ FACE" molded nameplate, with the separate "Dallas-Arbiter England" badge cropped out. "FUZZ FACE" is the product's own screen-printed name, not a separate logo, but it's a judgment call whether that's in-bounds — flagging for a human decision. |
| Modern | SWR bass | skipped | — | — | — | — | — | No usable SWR bass head photo found on Commons. |
| British | Trace Elliot bass amp | done (this pass) | amp_trace_elliot.jpg | Trace elliot vtype300.jpg | Avaktar at English Wikipedia | Public domain | https://commons.wikimedia.org/wiki/File:Trace_elliot_vtype300.jpg | Small "TRACE ELLIOT" badge plate is faintly visible above the knob row at the very top edge of the crop — left in because the alternative crop lost too much of the knob row; worth a closer look if this matters. |
| Rock | Ampeg SVT bass amp | done (this pass, **weak**) | amp_ampeg_svt.jpg | Ampeg SVT Classic.jpg | Steen Elm | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:Ampeg_SVT_Classic.jpg | Tried 3 different Commons sources (SVT Classic, SVT-3 Pro rack, SVT-3 Pro + cabs); this is the least-bad — knob row is still small/dim in the source photo. Kept for coverage; flagged weak like Hiwatt. |
| Hiwatt (A1) | Hiwatt Custom 50 | done (prior pass, **weak**, redo attempted & reverted) | amp_hiwatt.jpg | Hiwatt Model DR103 100W Guitar Amplifier.jpg | Oneparadigm | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Hiwatt_Model_DR103_100W_Guitar_Amplifier.jpg | Attempted a redo this pass with a Rock & Roll Hall of Fame display-case photo of Pete Townshend's Hiwatt — came out darker/blurrier (glass reflections) than the original, so kept the original DR103 crop. Source photo itself is only 440×277px; low-res is the ceiling here. |
| Brit Master Vol (A2) | '78 Marshall Mstr Volume | done (this pass) — **flag for review** | amp_marshall_plexi.jpg | Marshall 1959 Super Lead 100 Reissue Front.jpg | Roadside Guitars | CC BY-SA 2.0 | https://commons.wikimedia.org/wiki/File:Marshall_1959_Super_Lead_100_Reissue_Front.jpg | **Substitution**: no '78 JMP master-volume (metal-panel) photo found; used a 1959 Super Lead plexi-panel reissue instead — same brand/knob style but a visually distinct era/panel material from the real '78 model. Also has a non-Marshall novelty sticker ("The Devil Himself") on the head, outside the crop. |
| Brit 800 EL84 (A3) | '81 Marshall JCM800 with EL34s | done (prior pass) | amp_marshall_jcm800.jpg | Marshall JCM 800 model 2205 on Bench for bias adjustment (2009-05-21 04.40.54 by Roadside Guitars).jpg | Roadside Guitars | CC BY-SA 2.0 | https://commons.wikimedia.org/wiki/File:Marshall_JCM_800_model_2205_on_Bench_for_bias_adjustment_(2009-05-21_04.40.54_by_Roadside_Guitars).jpg | |
| Band Master (A4) | '72 Fender Bandmaster | done (this pass) | amp_fender_bandmaster.jpg | Fender Bandmaster Amp.jpg | Centervolume | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Fender_Bandmaster_Amp.jpg | Photo is actually an undated Bandmaster, not confirmed '72; visually consistent with the era either way. |
| Bass Man (A5) | '65 Fender Bassman | skipped | — | — | — | — | — | No usable Fender Bassman head/panel photo found (several searches; only tiny or unrelated/interior shots turned up). |
| Stella Bass (A6) | SWR Interstellar Odrive | skipped | — | — | — | — | — | No Commons results at all for this model. |
| '83 Concert (A7) | '83 Fender Concert Head | done (this pass) | amp_fender_concert.jpg | Fender Concert Amp.jpg | Centervolume | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Fender_Concert_Amp.jpg | Photo is a Concert combo, not confirmed '83/head version; same panel style. |
| Direct (A8) | Direct - no modelling | skipped (intentional) | — | — | — | — | — | Bypass/DI mode, not a physical amp — no image needed. |

**Amp totals: 15 done (13 solid + 2 flagged-for-review substitutions), 2 weak, 7 no-source-found, 1 intentional skip = 25.**

## Cabinet types (`src/jstation/data/dsp/cabinet.rs`)

| Nick | Real-world name | Status | Final file | Commons file title | Artist | License | Source URL | Notes |
|---|---|---|---|---|---|---|---|---|
| No Cabinet | No Cabinet | skipped (intentional) | — | — | — | — | — | Bypass mode — no image needed. |
| British 4x12 | Marshall 1960A with 75W Celestions | done (this pass — **redo of prior gap**) | cab_british4x12.jpg | Sovtek Mig 100H & Marshall 1960A Cab, Recording Fischer, Compound Recordings, January 2008.jpg | Roadside Guitars | CC BY-SA 2.0 | https://commons.wikimedia.org/wiki/File:Sovtek_Mig_100H_%26_Marshall_1960A_Cab,_Recording_Fischer,_Compound_Recordings,_January_2008.jpg | The prior pass found no usable source; this one (a recording-studio photo with a Sovtek head sitting on the 1960A cab) gives a clean grille+corner crop with no logo in frame. |
| Johnson 4x12 | loaded with Vintage 30W Celestions (generic) | done (this pass) | cab_johnson4x12.jpg | *(same source as British 4x12, different region)* | Roadside Guitars | CC BY-SA 2.0 | https://commons.wikimedia.org/wiki/File:Sovtek_Mig_100H_%26_Marshall_1960A_Cab,_Recording_Fischer,_Compound_Recordings,_January_2008.jpg | **Generic stand-in, reused source**: "Johnson" here just means Johnson's own cab loaded with that speaker, not a specific branded cab — reused the same Marshall 1960A photo from a different crop region since no separate generic 4x12 source was found. |
| Fane 4x12 | Hiwatt SE4123 with Fanes | skipped | — | — | — | — | — | No Commons results for Hiwatt SE4123 or "Hiwatt cabinet". |
| Johnson 2x12 | Open back with Vintage 30W Celestions (generic) | done (this pass) | cab_johnson2x12.jpg | *(same source as American 2x12, different region)* | Paulus 2 | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Fender_Twin_Reverb_Blonde.jpg | **Generic stand-in, reused source**: open-back 2x12 combo grille, cropped from a different region of the Fender Twin photo than the American 2x12 card uses. |
| American 2x12 | Fender Twin 2x12 | done (prior pass) | cab_american2x12.jpg | Fender Twin Reverb Blonde.jpg | Paulus 2 | CC BY-SA 4.0 | https://commons.wikimedia.org/wiki/File:Fender_Twin_Reverb_Blonde.jpg | |
| Jennings Blue 2x12 | '63 Vox AC30 | done (prior pass) | cab_jenningsblue2x12.jpg | VOX AC30 CC2 - panel-controls.jpg | massygo from Ghent, Belgium (edit by user:Retired electrician) | CC BY 2.0 | https://commons.wikimedia.org/wiki/File:VOX_AC30_CC2_-_panel-controls.jpg | |
| Tweed 1x12 | Fender Deluxe 1x12 | done (this pass) | cab_tweed1x12.jpg | Fender1953Deluxe.jpg | Johnny Bgood123 | CC BY-SA 3.0 de | https://commons.wikimedia.org/wiki/File:Fender1953Deluxe.jpg | Different crop region from the amp "Tweed" card, which uses the same source photo. |
| Blonde 2x12 | Bassman 2x12 | skipped | — | — | — | — | — | No usable Bassman piggyback/2x12 cabinet photo found. |
| Bass 4x10 with Tweeter | SWR 4x10 with tweeter | skipped | — | — | — | — | — | No Commons results for SWR 4x10 cabinet. |
| Bass 360 1x18 | Acoustic 360 | skipped | — | — | — | — | — | No Commons results for the Acoustic 360/Acoustic Control Corp cabinet, despite several search attempts — surprising given how iconic it is. |
| Flex Bass 1x15 | Ampeg Portaflex | done (prior pass) | amp_ampeg_portaflex.jpg (filename kept from prior pass; it's a cabinet-category card on the page) | Ampeg Portaflex B-15N (front).jpg | Ampeg Portaflex B-15N.JPG: Madlee; derivative work: Clusternote | CC BY-SA 3.0 | https://commons.wikimedia.org/wiki/File:Ampeg_Portaflex_B-15N_(front).jpg | |
| Green Back 4x12 | Marshall 1960B with 25W Celestion Greenbacks | skipped | — | — | — | — | — | No Commons results for "Marshall 1960B" specifically; did not reuse the 1960A photo again to avoid a third near-duplicate Marshall cab card. |
| Mega 1516 | Peavy 1x15 and 2x8 | skipped | — | — | — | — | — | No Commons results; unusual/rare combo cabinet. |
| Boutique 4x12 | VHT 4x12 with Celestion V30s | skipped | — | — | — | — | — | No Commons results for VHT cabinets. |
| '65 Tweed 1x12 | '65 Fender Deluxe | done (this pass) | cab_65tweed1x12.jpg | Fender1965DeluxeRev.jpg | Johnny Bgood123 | CC BY-SA 3.0 de | https://commons.wikimedia.org/wiki/File:Fender1965DeluxeRev.jpg | Despite the "Tweed" in the nick, a real '65 Deluxe Reverb is black tolex/silver grille, not tweed cloth — used the actual blackface-era look rather than forcing tweed cloth. |
| Goliath 4x10 | SWR Goliath | skipped | — | — | — | — | — | No Commons results for SWR Goliath. |
| Ivy League 1x10 | Fender Harvard | done (this pass) | cab_ivyleague1x10.jpg | Fender Harvard 1.jpg | Mfretton | CC0 | https://commons.wikimedia.org/wiki/File:Fender_Harvard_1.jpg | |
| Bass Man 4x10 | Fender Bassman | skipped | — | — | — | — | — | No usable Fender Bassman 4x10 cabinet photo found (same searches as Blonde 2x12 and the Bass Man amp entry all came up empty for this gear). |

**Cabinet totals: 9 done (7 solid + 2 generic reused-source stand-ins), 0 weak, 9 no-source-found, 1 intentional skip = 19.**

## Overall

- **Amps: 25 total** — 15 done, 2 weak, 7 no-source-found (+1 intentional skip for Direct).
- **Cabinets: 19 total** — 9 done, 0 weak, 9 no-source-found (+1 intentional skip for No Cabinet).
- **Grand total: 44 entries** — 24 done, 2 weak, 16 no-source-found, 2 intentional skips.

## Flag for human decision

1. **Fuzz (amp_fuzz_fuzzface.jpg)** — substituting a fuzz pedal photo for what is nominally an amp-voicing slot, cropped to keep the pedal's own "FUZZ FACE" nameplate in frame. Reasonable given "60's fuzz tone" names no amp, but worth a sanity check before committing to the repo.
2. **Brit Master Vol / amp_marshall_plexi.jpg** — stands in for a '78 Marshall master-volume amp with a 1959-style plexi-panel reissue photo; same brand and knob language, different era/panel material from the real model being modeled.
3. Two "weak" amp cards (Hiwatt, Ampeg SVT/Rock) are usable but visibly lower quality than the rest of the set — worth deciding whether to ship them as-is, keep hunting later, or go illustrated/vector for just these two.
