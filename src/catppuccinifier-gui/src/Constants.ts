
export const ConversionMethods = {
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
    latte: "Latte",
    frappe: "Frappe",
    macchiato: "Macchiato",
    mocha: "Mocha"
}

export const Accents = {
    rosewater: "Rosewater",
    flamingo: "Flamingo",
    pink: "Pink",
    mauve: "Mauve",
    red: "Red",
    maroon: "Maroon",
    peach: "Peach",
    yellow: "Yellow",
    green: "Green",
    teal: "Teal",
    sky: "Sky",
    sapphire: "Sapphire",
    blue: "Blue",
    lavender: "Lavender"
}
