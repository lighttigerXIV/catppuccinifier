#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GenerationData{
    pub image_path: String,
    pub hald: u8,
    pub luminosity: f64,
    pub algorithm: String,
    pub shape: f64,
    pub nearest: usize,
    pub mean: f64,
    pub std: f64,
    pub iterations: usize,
    pub power: f64,
    pub flavor: String
}