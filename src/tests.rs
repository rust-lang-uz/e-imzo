#[cfg(feature = "_daemon-on")]
#[cfg(test)]
mod daemon_on {
    use crate::EIMZO;
    use anyhow::{Context, Result};

    // TODO: how can we connect e-imzo service once instead of connecting on every function
    // const eimzo: EIMZO<Connected> = EIMZO::new().expect("Cannot connect to e-imzo.service");

    // check whather connection is successfull when e-imzo.service
    // enable on local machines
    #[test]
    fn check_connection() -> Result<()> {
        EIMZO::new().context("Cannot connect to e-imzo.service")?;
        Ok(())
    }
    #[test]
    fn list_all_certificates() -> Result<()> {
        let mut eimzo = EIMZO::new().context("Cannot connect to e-imzo.service")?;
        eimzo
            .list_all_certificates()
            .context("Cannot get certificates")?;

        Ok(())
    }
    #[test]
    fn change_ui_lang() -> Result<()> {
        let mut eimzo = EIMZO::new().context("Cannot connect to e-imzo.service")?;
        eimzo
            .change_ui_lang("uz")
            .context("Cannot change language")?;

        Ok(())
    }
    #[test]
    fn get_jvm_version() -> Result<()> {
        let mut eimzo = EIMZO::new().context("Cannot connect to e-imzo.service")?;
        eimzo.get_jvm_version().context("Cannot get jvm version")?;

        Ok(())
    }
}

#[cfg(feature = "_daemon-off")]
#[cfg(test)]
mod daemon_off {
    use crate::EIMZO;
    use anyhow::Context;

    #[test]
    fn check_off_connection() {
        let eimzo = EIMZO::new().context("E-IMZO service is down good :)");
        assert!(eimzo.is_err());
    }
}
