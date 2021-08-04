use std::ops::Range;

// field lengths in decoded characters
pub const FLARM_ID_LENGTH: usize = 6;
pub const PILOT_NAME_LENGTH: usize = 21;
pub const AIRFIELD_LENGTH: usize = 21;
pub const PLANE_TYPE_LENGTH: usize = 21;
pub const REGISTRATION_LENGTH: usize = 7;
pub const CALL_SIGN_LENGTH: usize = 3;
pub const FREQUENCY_LENGTH: usize = 7;

// field offsets in bytes
pub const FLARM_ID_START: usize = 0;
pub const PILOT_NAME_START: usize = FLARM_ID_START + FLARM_ID_LENGTH * 2;
pub const AIRFIELD_START: usize = PILOT_NAME_START + PILOT_NAME_LENGTH * 2;
pub const PLANE_TYPE_START: usize = AIRFIELD_START + AIRFIELD_LENGTH * 2;
pub const REGISTRATION_START: usize = PLANE_TYPE_START + PLANE_TYPE_LENGTH * 2;
pub const CALL_SIGN_START: usize = REGISTRATION_START + REGISTRATION_LENGTH * 2;
pub const FREQUENCY_START: usize = CALL_SIGN_START + CALL_SIGN_LENGTH * 2;

pub const LINE_LENGTH: usize = FREQUENCY_START + FREQUENCY_LENGTH * 2;

// field ranges in bytes
pub const FLARM_ID_RANGE: Range<usize> = FLARM_ID_START..PILOT_NAME_START;
pub const PILOT_NAME_RANGE: Range<usize> = PILOT_NAME_START..AIRFIELD_START;
pub const AIRFIELD_RANGE: Range<usize> = AIRFIELD_START..PLANE_TYPE_START;
pub const PLANE_TYPE_RANGE: Range<usize> = PLANE_TYPE_START..REGISTRATION_START;
pub const REGISTRATION_RANGE: Range<usize> = REGISTRATION_START..CALL_SIGN_START;
pub const CALL_SIGN_RANGE: Range<usize> = CALL_SIGN_START..FREQUENCY_START;
pub const FREQUENCY_RANGE: Range<usize> = FREQUENCY_START..LINE_LENGTH;
