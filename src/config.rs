use serde::{Serialize, Deserialize};

#[derive( Debug, Serialize, Deserialize)]
pub struct Config{
    pub fiat: String,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            fiat: "USD".to_string(),
        }
    }
}

pub fn current() -> Config {
    match confy::load::<Config>("stf", "config") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Erro ao carregar a configuração: {}", e);
            Config::default() // Retorna a configuração padrão em caso de erro
        }
    }
}

//TODO: create a function to set a configuration
pub fn _set(config: String) {
    if let Err(e) = (|| -> Result<(), Box<dyn std::error::Error>> {
        let mut cfg: Config = confy::load("stf", "config")?;
        cfg = Config {
            fiat: config.to_string(),
            ..cfg
        };
        confy::store("stf", "config", &cfg)?;
        Ok(())
    })() {
        eprintln!("Erro ao definir a configuração: {}", e);
    }
}

