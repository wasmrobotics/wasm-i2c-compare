use crate::bindings::wasm_robotics::robotics::imus::CalibrationData;

impl PartialEq<CalibrationData> for CalibrationData {
    fn ne(&self, other: &CalibrationData) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &CalibrationData) -> bool {
        self.acc_offset_x_lsb == other.acc_offset_x_lsb
            && self.acc_offset_x_msb == other.acc_offset_x_msb
            && self.acc_offset_y_lsb == other.acc_offset_y_lsb
            && self.acc_offset_y_msb == other.acc_offset_y_msb
            && self.acc_offset_z_lsb == other.acc_offset_z_lsb
            && self.acc_offset_z_msb == other.acc_offset_z_msb
            && self.mag_offset_x_lsb == other.mag_offset_x_lsb
            && self.mag_offset_x_msb == other.mag_offset_x_msb
            && self.mag_offset_y_lsb == other.mag_offset_y_lsb
            && self.mag_offset_y_msb == other.mag_offset_y_msb
            && self.mag_offset_z_lsb == other.mag_offset_z_lsb
            && self.mag_offset_z_msb == other.mag_offset_z_msb
            && self.gyr_offset_x_lsb == other.gyr_offset_x_lsb
            && self.gyr_offset_x_msb == other.gyr_offset_x_msb
            && self.gyr_offset_y_lsb == other.gyr_offset_y_lsb
            && self.gyr_offset_y_msb == other.gyr_offset_y_msb
            && self.gyr_offset_z_lsb == other.gyr_offset_z_lsb
            && self.gyr_offset_z_msb == other.gyr_offset_z_msb
            && self.acc_radius_lsb == other.acc_radius_lsb
            && self.acc_radius_msb == other.acc_radius_msb
            && self.mag_radius_lsb == other.mag_radius_lsb
            && self.mag_radius_msb == other.mag_radius_msb
    }
}

pub fn serialize(calibration_data: &CalibrationData) -> [u8; 22] {
    let mut result: [u8; 22] = [0; 22];
    result[0] = calibration_data.acc_offset_x_lsb;
    result[1] = calibration_data.acc_offset_x_msb;
    result[2] = calibration_data.acc_offset_y_lsb;
    result[3] = calibration_data.acc_offset_y_msb;
    result[4] = calibration_data.acc_offset_z_lsb;
    result[5] = calibration_data.acc_offset_z_msb;
    result[6] = calibration_data.mag_offset_x_lsb;
    result[7] = calibration_data.mag_offset_x_msb;
    result[8] = calibration_data.mag_offset_y_lsb;
    result[9] = calibration_data.mag_offset_y_msb;
    result[10] = calibration_data.mag_offset_z_lsb;
    result[11] = calibration_data.mag_offset_z_msb;
    result[12] = calibration_data.gyr_offset_x_lsb;
    result[13] = calibration_data.gyr_offset_x_msb;
    result[14] = calibration_data.gyr_offset_y_lsb;
    result[15] = calibration_data.gyr_offset_y_msb;
    result[16] = calibration_data.gyr_offset_z_lsb;
    result[17] = calibration_data.gyr_offset_z_msb;
    result[18] = calibration_data.acc_radius_lsb;
    result[19] = calibration_data.acc_radius_msb;
    result[20] = calibration_data.mag_radius_lsb;
    result[21] = calibration_data.mag_radius_msb;
    result
}

pub fn deserialize(data: &[u8; 22]) -> CalibrationData {
    let result = CalibrationData {
        acc_offset_x_lsb: data[0],
        acc_offset_x_msb: data[1],
        acc_offset_y_lsb: data[2],
        acc_offset_y_msb: data[3],
        acc_offset_z_lsb: data[4],
        acc_offset_z_msb: data[5],
        mag_offset_x_lsb: data[6],
        mag_offset_x_msb: data[7],
        mag_offset_y_lsb: data[8],
        mag_offset_y_msb: data[9],
        mag_offset_z_lsb: data[10],
        mag_offset_z_msb: data[11],
        gyr_offset_x_lsb: data[12],
        gyr_offset_x_msb: data[13],
        gyr_offset_y_lsb: data[14],
        gyr_offset_y_msb: data[15],
        gyr_offset_z_lsb: data[16],
        gyr_offset_z_msb: data[17],
        acc_radius_lsb: data[18],
        acc_radius_msb: data[19],
        mag_radius_lsb: data[20],
        mag_radius_msb: data[21],
    };

    result
}
#[cfg(test)]
mod test_super {

    use super::*;

    #[test]
    fn test_calibration_serialize() {
        let calibration_data = CalibrationData {
            acc_offset_x_lsb: 0xD,
            acc_offset_x_msb: 0xE,
            acc_offset_y_lsb: 0xA,
            acc_offset_y_msb: 0xD,
            acc_offset_z_lsb: 0xB,
            acc_offset_z_msb: 0xE,
            mag_offset_x_lsb: 0xE,
            mag_offset_x_msb: 0xF,
            mag_offset_y_lsb: 0xB,
            mag_offset_y_msb: 0xA,
            mag_offset_z_lsb: 0xD,
            mag_offset_z_msb: 0xD,
            gyr_offset_x_lsb: 0xE,
            gyr_offset_x_msb: 0xC,
            gyr_offset_y_lsb: 0xA,
            gyr_offset_y_msb: 0xF,
            gyr_offset_z_lsb: 0xC,
            gyr_offset_z_msb: 0x0,
            acc_radius_lsb: 0xD,
            acc_radius_msb: 0xE,
            mag_radius_lsb: 0x0,
            mag_radius_msb: 0x0,
        };

        let result = serialize(&calibration_data);
        assert_eq!(
            [
                0xD, 0xE, 0xA, 0xD, 0xB, 0xE, 0xE, 0xF, 0xB, 0xA, 0xD, 0xD, 0xE, 0xC, 0xA, 0xF,
                0xC, 0x0, 0xD, 0xE, 0x0, 0x0
            ],
            result
        );
    }

    #[test]
    fn test_calibration_deserialize() {
        let data: [u8; 22] = [
            0xD, 0xE, 0xA, 0xD, 0xB, 0xE, 0xE, 0xF, 0xB, 0xA, 0xD, 0xD, 0xE, 0xC, 0xA, 0xF, 0xC,
            0x0, 0xD, 0xE, 0x0, 0x0,
        ];
        let expected = CalibrationData {
            acc_offset_x_lsb: 0xD,
            acc_offset_x_msb: 0xE,
            acc_offset_y_lsb: 0xA,
            acc_offset_y_msb: 0xD,
            acc_offset_z_lsb: 0xB,
            acc_offset_z_msb: 0xE,
            mag_offset_x_lsb: 0xE,
            mag_offset_x_msb: 0xF,
            mag_offset_y_lsb: 0xB,
            mag_offset_y_msb: 0xA,
            mag_offset_z_lsb: 0xD,
            mag_offset_z_msb: 0xD,
            gyr_offset_x_lsb: 0xE,
            gyr_offset_x_msb: 0xC,
            gyr_offset_y_lsb: 0xA,
            gyr_offset_y_msb: 0xF,
            gyr_offset_z_lsb: 0xC,
            gyr_offset_z_msb: 0x0,
            acc_radius_lsb: 0xD,
            acc_radius_msb: 0xE,
            mag_radius_lsb: 0x0,
            mag_radius_msb: 0x0,
        };

        let result = deserialize(&data);
        assert_eq!(expected, result);
    }
}
