
export const Algorithms = {
    gaussian: {
        codename: "gaussian",
        name: "Gaussian RBF (Recommended)",
        properties: {
            shape: {
                default: 96,
                min: 0,
                max: 512
            },
            nearest: {
                default: 26,
                min: 2,
                max: 26
            }
        }
    },
    gaussian_sampling: {
        codename: "gaussian_sampling",
        name: "Gaussian Sampling",
        properties: {
            mean: {
                default: 0,
                min: 0,
                max: 255
            },
            std: {
                default: 20,
                min: 0,
                max: 255
            },
            iterations: {
                default: 512,
                min: 0,
                max: 1024
            }
        }
    },
    linear: {
        codename: "linear",
        name: "Linear",
        properties: {
            nearest: {
                default: 5,
                min: 0,
                max: 26
            }
        }
    },
    sheppard: {
        codename: "sheppard",
        name: "Sheppard's Method",
        properties: {
            power: {
                default: 4,
                min: 0,
                max: 64
            },
            nearest: {
                default: 26,
                min: 2,
                max: 26
            }
        }
    },
    nearest_neighbor: {
        codename: "nearest_neighbour",
        name: "Nearest Neighbour"
    }
}

export const Themes = {
    latte: "latte",
    frappe: "frappe",
    macchiato: "macchiato",
    mocha: "mocha"
}

export const Accents = {
    rosewater: "rosewater",
    flamingo: "flamingo",
    pink: "pink",
    mauve: "mauve",
    red: "red",
    maroon: "maroon",
    peach: "peach",
    yellow: "yellow",
    green: "green",
    teal: "teal",
    sky: "sky",
    sapphire: "sapphire",
    blue: "blue",
    lavender: "lavender"
}

export interface GenerationData {
    image_path: string,
    hald: number,
    luminosity: number,
    algorithm: string,
    shape: number,
    nearest: number,
    mean: number,
    std: number,
    iterations: number,
    power: number,
    flavor: string
}

export interface GenerateImagesData {
    imagePath: string,
    hald: number,
    luminosity: number,
    algorithm: string,
    shape: number,
    nearest: number,
    mean: number,
    std: number,
    iterations: number,
    power: number,
}

export interface ImagePaths{
    selected:{
        local: string,
        raw: string
    },
    latte:{
        local: string,
        raw: string
    },
    frappe:{
        local: string,
        raw: string
    },
    macchiato:{
        local: string,
        raw: string
    },
    mocha:{
        local: string,
        raw: string
    },
    oled:{
        local: string,
        raw: string
    }
}