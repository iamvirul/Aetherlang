#[cfg(test)]
mod tests {
    use crate::core::datetime::AetherDateTime;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_datetime_now() {
        let dt = AetherDateTime::now();
        assert!(dt.timestamp() > 0);
        
        // Test that two consecutive now() calls give different or equal timestamps
        let dt2 = AetherDateTime::now();
        assert!(dt2.timestamp() >= dt.timestamp());
    }

    #[test]
    fn test_from_timestamp() {
        // Test normal case
        let timestamp = 1609459200; // 2021-01-01 00:00:00 UTC
        let dt = AetherDateTime::from_timestamp(timestamp).unwrap();
        assert_eq!(dt.year(), 2021);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 0);
        assert_eq!(dt.minute(), 0);
        assert_eq!(dt.second(), 0);

        // Test edge cases
        // Test very old date
        let old_timestamp = -2208988800; // 1900-01-01 00:00:00 UTC
        let old_dt = AetherDateTime::from_timestamp(old_timestamp).unwrap();
        assert_eq!(old_dt.year(), 1900);
        
        // Test future date
        let future_timestamp = 32503680000; // 3000-01-01 00:00:00 UTC
        let future_dt = AetherDateTime::from_timestamp(future_timestamp).unwrap();
        assert_eq!(future_dt.year(), 3000);
    }

    #[test]
    fn test_components() {
        let dt = AetherDateTime::from_timestamp(1609459200).unwrap();
        assert_eq!(dt.year(), 2021);
        assert_eq!(dt.month(), 1);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 0);
        assert_eq!(dt.minute(), 0);
        assert_eq!(dt.second(), 0);

        // Test last day of month
        let dt = Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 59).unwrap();
        let dt = AetherDateTime::from_timestamp(dt.timestamp()).unwrap();
        assert_eq!(dt.year(), 2021);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
        assert_eq!(dt.second(), 59);
    }

    #[test]
    fn test_format() {
        let dt = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let dt = AetherDateTime::from_timestamp(dt.timestamp()).unwrap();
        
        // Test different format strings
        assert_eq!(dt.format("%Y-%m-%d"), "2021-01-01");
        assert_eq!(dt.format("%d/%m/%Y"), "01/01/2021");
        assert_eq!(dt.format("%H:%M:%S"), "00:00:00");
        assert_eq!(dt.format("%Y-%m-%d %H:%M:%S"), "2021-01-01 00:00:00");
        
        // Test with different components
        let dt = Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 59).unwrap();
        let dt = AetherDateTime::from_timestamp(dt.timestamp()).unwrap();
        assert_eq!(dt.format("%Y-%m-%d %H:%M:%S"), "2021-12-31 23:59:59");
    }

    #[test]
    fn test_display() {
        // Test start of year
        let dt = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
        let dt = AetherDateTime::from_timestamp(dt.timestamp()).unwrap();
        assert_eq!(dt.to_string(), "2021-01-01 00:00:00 UTC");

        // Test end of year
        let dt = Utc.with_ymd_and_hms(2021, 12, 31, 23, 59, 59).unwrap();
        let dt = AetherDateTime::from_timestamp(dt.timestamp()).unwrap();
        assert_eq!(dt.to_string(), "2021-12-31 23:59:59 UTC");
    }

    #[test]
    fn test_to_local() {
        let dt = AetherDateTime::now();
        let local = dt.to_local();
        
        // Local time should be within 24 hours of UTC
        let diff = (local.timestamp() - dt.timestamp()).abs();
        assert!(diff <= 24 * 60 * 60);
    }
} 