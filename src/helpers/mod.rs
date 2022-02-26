use crate::*;

macro_rules! sample_errors {
    ($dev: tt, $wait: block) => {{
        let mut maybe_not_started_error = 0;
        let mut whole_error = 0;

        for n in 1..6 {
            debug!("sample error: {}", n);

            let _ = $dev.result();
            if let Err(_) = $dev.status() {
                whole_error += 1;

                match $dev.error_id() {
                    Ok(id) => {
                        if id.has_error(DeviceError::ReadRegisterInvalid) {
                            maybe_not_started_error += 1
                        }
                    }
                    Err(_) => {}
                }
            }
            $wait;
        }

        (whole_error, maybe_not_started_error)
    }};
}

#[cfg(feature = "std")]
#[cfg(feature = "with_tokio")]
pub async fn resume_or_restart<T: Ccs811>(
    dev: &mut T,
    mode: MeasureDriveMode,
    interrupt: MeasureInterrupt,
    thresh: MeasureThresh,
) -> Ccs811Result<()> {
    let (whole_error, maybe_not_started_error) = sample_errors!(dev, {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
    });

    if whole_error == 5 || maybe_not_started_error >= 4 {
        debug!("restart");
        dev.start(mode, interrupt, thresh)
    } else {
        debug!("resume");
        Ok(())
    }
}

#[cfg(feature = "std")]
#[cfg(not(feature = "with_tokio"))]
pub fn resume_or_restart<T: Ccs811>(
    dev: &mut T,
    mode: MeasureDriveMode,
    interrupt: MeasureInterrupt,
    thresh: MeasureThresh,
) -> Ccs811Result<()> {
    let (whole_error, maybe_not_started_error) = sample_errors!(dev, {
        std::thread::sleep(std::time::Duration::from_secs(1))
    });

    if whole_error == 5 || maybe_not_started_error >= 4 {
        debug!("restart");
        dev.start(mode, interrupt, thresh)
    } else {
        debug!("resume");
        Ok(())
    }
}
