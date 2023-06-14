import { useState } from "react";
import { open, save } from "@tauri-apps/api/dialog"
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { WebviewWindow } from "@tauri-apps/api/window"
import { ConversionMethods } from "./Constants";

export function AppViewModel() {

    const [theme, setTheme] = useState(localStorage.getItem("theme") !== null ? localStorage.getItem("theme") : "theme-mocha")
    const [accent, setAccent] = useState(localStorage.getItem("accent") !== null ? localStorage.getItem("accent") : "blue")
    const [haldLevel, setHaldLevel] = useState(4);
    const [conversionMethod, setConversionMethod] = useState(ConversionMethods.gaussian.codename);
    const [showAdvancedConversion, setShowAdvancedConversion] = useState(false);
    const [gaussianEuclide, setGaussianEuclide] = useState(ConversionMethods.gaussian.properties.euclide.default);
    const [gaussianNearest, setGaussianNearest] = useState(ConversionMethods.gaussian.properties.nearest.default);
    const [gaussianSamplingMean, setGaussianSamplingMean] = useState(ConversionMethods.gaussian_sampling.properties.mean.default);
    const [gaussianSamplingSTD, setGaussianSamplingSTD] = useState(ConversionMethods.gaussian_sampling.properties.std.default);
    const [gaussianSamplingIterations, setGaussianSamplingIterations] = useState(ConversionMethods.gaussian_sampling.properties.iterations.default);
    const [linearNearest, setLinearNearest] = useState(ConversionMethods.linear.properties.nearest.default);
    const [sheppardPower, setSheppardPower] = useState(ConversionMethods.sheppard.properties.power.default);
    const [sheppardNearest, setSheppardNearest] = useState(ConversionMethods.sheppard.properties.nearest.default);
    const [selectedImageRawPath, setSelectedImageRawPath] = useState("");
    const [selectedImagePath, setSelectedImagePath] = useState("");
    const [showGeneratedGrids, setShowGeneratedGrids] = useState(false);
    const [generatedLatteRawPath, setGeneratedLatteRawPath] = useState("");
    const [generatedLattePath, setGeneratedLattePath] = useState("");
    const [generatedFrappeRawPath, setGeneratedFrappeRawPath] = useState("");
    const [generatedFrappePath, setGeneratedFrappePath] = useState("");
    const [generatedMacchiatoRawPath, setGeneratedMacchiatoRawPath] = useState("");
    const [generatedMacchiatoPath, setGeneratedMacchiatoPath] = useState("");
    const [generatedMochaRawPath, setGeneratedMochaRawPath] = useState("");
    const [generatedMochaPath, setGeneratedMochaPath] = useState("");
    const [generatedOledRawPath, setGeneratedOledRawPath] = useState("");
    const [generatedOledPath, setGeneratedOledPath] = useState("");
    const [generatingImages, setGeneratingImages] = useState(false)
    const [showSettings, setShowSettings] = useState(false);
    const [showSideNav, setShowSideNav] = useState(false);

    async function selectImage() {

        const selected = await open({
            multiple: false,
            filters: [{
                name: "Images",
                extensions: ["png", "webp", "jpg", "jpeg"]
            }]
        });

        if (selected !== null) {

            setSelectedImageRawPath(selected);
            setSelectedImagePath(convertFileSrc(selected));
            setGeneratedLattePath("");
            setGeneratedFrappePath("");
            setGeneratedMacchiatoPath("");
            setGeneratedMochaPath("");
            setGeneratedOledPath("");
            setShowGeneratedGrids(false);
        }
    }

    async function generateImages() {

        setGeneratedLattePath("");
        setGeneratedFrappePath("");
        setGeneratedMacchiatoPath("");
        setGeneratedMochaPath("");
        setGeneratedOledPath("");

        setGeneratingImages(true);

        setShowGeneratedGrids(true);

        await invoke("clear_temp_folder");

        await invoke("generate_image", {
            image_path: selectedImageRawPath,
            hald_level: haldLevel,
            flavor: "latte",
            conversion_method: conversionMethod,
            gaussian_euclide: gaussianEuclide,
            gaussian_nearest: gaussianNearest,
            gaussian_sampling_mean: gaussianSamplingMean,
            gaussian_sampling_std: gaussianSamplingSTD,
            gaussian_sampling_iterations: gaussianSamplingIterations,
            linear_nearest: linearNearest,
            sheppard_power: sheppardPower,
            sheppard_nearest: sheppardNearest
        })
            .then(path => {

                setGeneratedLatteRawPath(path);
                setGeneratedLattePath(convertFileSrc(path));
            })
            .catch((error) => { console.error(error) });

        await invoke("generate_image", {
            image_path: selectedImageRawPath,
            hald_level: haldLevel,
            flavor: "frappe",
            conversion_method: conversionMethod,
            gaussian_euclide: gaussianEuclide,
            gaussian_nearest: gaussianNearest,
            gaussian_sampling_mean: gaussianSamplingMean,
            gaussian_sampling_std: gaussianSamplingSTD,
            gaussian_sampling_iterations: gaussianSamplingIterations,
            linear_nearest: linearNearest,
            sheppard_power: sheppardPower,
            sheppard_nearest: sheppardNearest
        })
            .then(path => {

                setGeneratedFrappeRawPath(path);
                setGeneratedFrappePath(convertFileSrc(path));
            })
            .catch((error) => { console.error(error) });

        await invoke("generate_image", {
            image_path: selectedImageRawPath,
            hald_level: haldLevel,
            flavor: "macchiato",
            conversion_method: conversionMethod,
            gaussian_euclide: gaussianEuclide,
            gaussian_nearest: gaussianNearest,
            gaussian_sampling_mean: gaussianSamplingMean,
            gaussian_sampling_std: gaussianSamplingSTD,
            gaussian_sampling_iterations: gaussianSamplingIterations,
            linear_nearest: linearNearest,
            sheppard_power: sheppardPower,
            sheppard_nearest: sheppardNearest
        })
            .then(path => {

                setGeneratedMacchiatoRawPath(path);
                setGeneratedMacchiatoPath(convertFileSrc(path));
            })
            .catch((error) => { console.error(error) });

        await invoke("generate_image", {
            image_path: selectedImageRawPath,
            hald_level: haldLevel,
            flavor: "mocha",
            conversion_method: conversionMethod,
            gaussian_euclide: gaussianEuclide,
            gaussian_nearest: gaussianNearest,
            gaussian_sampling_mean: gaussianSamplingMean,
            gaussian_sampling_std: gaussianSamplingSTD,
            gaussian_sampling_iterations: gaussianSamplingIterations,
            linear_nearest: linearNearest,
            sheppard_power: sheppardPower,
            sheppard_nearest: sheppardNearest
        })
            .then(path => {

                setGeneratedMochaRawPath(path);
                setGeneratedMochaPath(convertFileSrc(path));
            })
            .catch((error) => { console.error(error) });

        await invoke("generate_image", {
            image_path: selectedImageRawPath,
            hald_level: haldLevel,
            flavor: "oled",
            conversion_method: conversionMethod,
            gaussian_euclide: gaussianEuclide,
            gaussian_nearest: gaussianNearest,
            gaussian_sampling_mean: gaussianSamplingMean,
            gaussian_sampling_std: gaussianSamplingSTD,
            gaussian_sampling_iterations: gaussianSamplingIterations,
            linear_nearest: linearNearest,
            sheppard_power: sheppardPower,
            sheppard_nearest: sheppardNearest
        })
            .then(path => {

                setGeneratedOledRawPath(path);
                setGeneratedOledPath(convertFileSrc(path));
            })
            .catch((error) => { console.error(error) });

        setGeneratingImages(false);
    }

    function saveImage(flavor) {

        var fileName = "";
        invoke("get_os")
            .then((os) => {

                if (os === "windows") {
                    fileName = flavor + "-noise" + haldLevel + "-" + selectedImageRawPath.split('\\').pop();
                } else {
                    fileName = flavor + "-noise" + haldLevel + "-" + selectedImageRawPath.split('/').pop();
                }

                save({
                    defaultPath: fileName,
                    filters: [
                        {
                            name: "Images (png, jpg, jpeg, webp)",
                            extensions: ["png", "webp", "jpg", "jpeg"]
                        }
                    ]
                }).then((savedPath) => {

                    switch (flavor) {
                        case "latte": {
                            invoke("save_image", { image_path: generatedLatteRawPath, saved_path: savedPath })
                            break;
                        }
                        case "frappe": {
                            invoke("save_image", { image_path: generatedFrappeRawPath, saved_path: savedPath })
                            break;
                        }
                        case "macchiato": {
                            invoke("save_image", { image_path: generatedMacchiatoRawPath, saved_path: savedPath })
                            break;
                        }
                        case "mocha": {
                            invoke("save_image", { image_path: generatedMochaRawPath, saved_path: savedPath })
                            break;
                        }
                        case "oled": {
                            invoke("save_image", { image_path: generatedOledRawPath, saved_path: savedPath })
                            break;
                        }
                    }
                })
            })
    }

    function previewImage(flavor, path) {
        const webview = new WebviewWindow('preview', {
            url: 'preview.html?path=' + path,
            title: flavor + " Preview",
        })

        webview.once('tauri://created', function () { })
        webview.once('tauri://error', function (e) {
            console.error(e);
        })
    }

    function resetConversionValues() {
        setGaussianEuclide(ConversionMethods.gaussian.properties.euclide.default);
        setGaussianNearest(ConversionMethods.gaussian.properties.nearest.default);
        setGaussianSamplingMean(ConversionMethods.gaussian_sampling.properties.mean.default);
        setGaussianSamplingSTD(ConversionMethods.gaussian_sampling.properties.std.default);
        setGaussianSamplingIterations(ConversionMethods.gaussian_sampling.properties.iterations.default);
        setLinearNearest(ConversionMethods.linear.properties.nearest.default);
        setSheppardPower(ConversionMethods.sheppard.properties.power.default);
        setSheppardNearest(ConversionMethods.sheppard.properties.nearest.default);
    }

    return {
        theme, setTheme,
        accent, setAccent,
        noiseLevel: haldLevel, setNoiseLevel: setHaldLevel,
        conversionMethod, setConversionMethod,
        showAdvancedConversion, setShowAdvancedConversion,
        gaussianEuclide, setGaussianEuclide,
        gaussianNearest, setGaussianNearest,
        gaussianSamplingMean, setGaussianSamplingMean,
        gaussianSamplingSTD, setGaussianSamplingSTD,
        gaussianSamplingIterations, setGaussianSamplingIterations,
        linearNearest, setLinearNearest,
        sheppardPower, setSheppardPower,
        sheppardNearest, setSheppardNearest,
        selectedImageRawPath, setSelectedImageRawPath,
        selectedImagePath, setSelectedImagePath,
        showGeneratedGrids, setShowGeneratedGrids,
        generatedLattePath, setGeneratedLattePath,
        generatedLatteRawPath, setGeneratedLatteRawPath,
        generatedFrappePath, setGeneratedFrappePath,
        generatedFrappeRawPath, setGeneratedFrappeRawPath,
        generatedMacchiatoPath, setGeneratedMacchiatoPath,
        generatedMacchiatoRawPath, setGeneratedMacchiatoRawPath,
        generatedMochaPath, setGeneratedMochaPath,
        generatedMochaRawPath, setGeneratedMochaRawPath,
        generatedOledPath, setGeneratedOledPath,
        generatedOledRawPath, setGeneratedOledRawPath,
        generatingImages, setGeneratingImages,
        showSettings, setShowSettings,
        showSideNav, setShowSideNav,
        selectImage,
        generateImages,
        saveImage,
        previewImage,
        resetConversionValues
    }
}