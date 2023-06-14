import { React } from "react";
import "./App.css";

import { open as openLink } from '@tauri-apps/api/shell';
import GeneratedImageBox from "./components/GeneratedImageBox";
import Switch from '@mui/material/Switch';

//Resources
import { ReactComponent as SettingsIcon } from "./assets/images/settings.svg"
import { ReactComponent as GitHubIcon } from "./assets/images/github.svg"
import { ReactComponent as CloseIcon } from "./assets/images/close.svg"
import { ReactComponent as SunflowerIcon } from "./assets/images/sunflower.svg"
import { ReactComponent as PineappleIcon } from "./assets/images/pineapple.svg"
import { ReactComponent as LotusIcon } from "./assets/images/lotus.svg"
import { ReactComponent as BranchIcon } from "./assets/images/branch.svg"
import { ReactComponent as MenuIcon } from "./assets/images/menu.svg"
import { AppViewModel } from "./AppVM";
import { ConversionMethods } from "./Constants";


function App() {

  const {
    theme, setTheme,
    accent, setAccent,
    noiseLevel, setNoiseLevel,
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
  } = AppViewModel()

  function themesSettingClasses(setting) {

    if (setting === theme) {
      return "bg-skin-base border-skin-surface1"
    } else {
      return "bg-skin-mantle border-skin-surface0"
    }
  }

  function accentSettingClasses(setting) {

    if (setting === accent) {
      return "bg-skin-base border-skin-surface1 col-span-1"
    } else {
      return "bg-skin-mantle border-skin-surface0 col-span-1"
    }
  }

  function getRadioColor(){

    return "checked:bg-skin-" + accent
  }

  function GenerateImageSection() {

    return (
      <div className="flex flex-col flex-grow p-4">
        <div className="p-2 bg-skin-mantle rounded-xl border border-skin-surface0">
          <div>Hald Level - {noiseLevel}</div>
          <input className={"w-full slider slider-" + accent} value={noiseLevel} onChange={e => { setNoiseLevel(parseInt(e.target.value)) }} type="range" max={8} min={2} />
        </div>

        <div className="mt-2 p-2 bg-skin-mantle rounded-xl border border-skin-surface0">

          <div>Conversion Method</div>

          <div className="flex">

            <div className="flex-grow">Advanced</div>

            <Switch
              checked={showAdvancedConversion}
              onChange={() => {
                setShowAdvancedConversion(!showAdvancedConversion)
                resetConversionValues()
              }}
              name=""
            />
          </div>

          <div className="mt-2 p-2 border border-skin-surface0 rounded-xl">
            <div
              className="flex items-center cursor-pointer"
              onClick={() => {
                setConversionMethod(ConversionMethods.gaussian.codename);
              }}
            >

              <div className="flex-grow font-semibold">{ConversionMethods.gaussian.name}</div>

              <input
                checked={conversionMethod === ConversionMethods.gaussian.codename}
                type="radio"
                className={" rounded-full bg-skin-base border border-skin-surface0 " + getRadioColor()}
              />
            </div>

            {
              conversionMethod === ConversionMethods.gaussian.codename && showAdvancedConversion ?
                <div>

                  <div>Euclide - {gaussianEuclide}</div>
                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={gaussianEuclide}
                    onChange={e => { setGaussianEuclide(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.gaussian.properties.euclide.max}
                    min={ConversionMethods.gaussian.properties.euclide.min}
                  />

                  <div>Nearest - {gaussianNearest}</div>
                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={gaussianNearest}
                    onChange={e => { setGaussianNearest(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.gaussian.properties.nearest.max}
                    min={ConversionMethods.gaussian.properties.nearest.min}
                  />
                </div>
                :
                null
            }
          </div>

          <div className="mt-2 p-2 border border-skin-surface0 rounded-xl">
            <div
              className="flex items-center cursor-pointer"
              onClick={() => {
                setConversionMethod(ConversionMethods.gaussian_sampling.codename);
              }}
            >

              <div className="flex-grow font-semibold">{ConversionMethods.gaussian_sampling.name}</div>

              <input
                checked={conversionMethod === ConversionMethods.gaussian_sampling.codename}
                type="radio"
                className={" rounded-full bg-skin-base border border-skin-surface0 " + getRadioColor()}
              />
            </div>

            {
              conversionMethod === ConversionMethods.gaussian_sampling.codename && showAdvancedConversion ?
                <div>

                  <div>Mean - {gaussianSamplingMean}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={gaussianSamplingMean}
                    onChange={e => { setGaussianSamplingMean(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.gaussian_sampling.properties.mean.max}
                    min={ConversionMethods.gaussian_sampling.properties.mean.min}
                  />

                  <div>STD - {gaussianSamplingSTD}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={gaussianSamplingSTD}
                    onChange={e => { setGaussianSamplingSTD(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.gaussian_sampling.properties.std.max}
                    min={ConversionMethods.gaussian_sampling.properties.std.min}
                  />

                  <div>Iterations - {gaussianSamplingIterations}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={gaussianSamplingIterations}
                    onChange={e => { setGaussianSamplingIterations(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.gaussian_sampling.properties.iterations.max}
                    min={ConversionMethods.gaussian_sampling.properties.iterations.min}
                  />
                </div>
                :
                null
            }
          </div>

          <div className="mt-2 p-2 border border-skin-surface0 rounded-xl">
            <div
              className="flex items-center cursor-pointer"
              onClick={() => {
                setConversionMethod(ConversionMethods.linear.codename);
              }}
            >

              <div className="flex-grow font-semibold">{ConversionMethods.linear.name}</div>

              <input
                checked={conversionMethod === ConversionMethods.linear.codename}
                type="radio"
                className={" rounded-full bg-skin-base border border-skin-surface0 " + getRadioColor()}
              />
            </div>

            {
              conversionMethod === ConversionMethods.linear.codename && showAdvancedConversion ?
                <div>

                  <div>Nearest - {linearNearest}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={linearNearest}
                    onChange={e => { setLinearNearest(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.linear.properties.nearest.max}
                    min={ConversionMethods.linear.properties.nearest.min}
                  />
                </div>
                :
                null
            }
          </div>

          <div className="mt-2 p-2 border border-skin-surface0 rounded-xl">
            <div
              className="flex items-center cursor-pointer"
              onClick={() => {
                setConversionMethod(ConversionMethods.sheppard.codename);
              }}
            >

              <div className="flex-grow font-semibold">{ConversionMethods.sheppard.name}</div>

              <input
                checked={conversionMethod === ConversionMethods.sheppard.codename}
                type="radio"
                className={" rounded-full bg-skin-base border border-skin-surface0 " + getRadioColor()}
              />
            </div>

            {
              conversionMethod === ConversionMethods.sheppard.codename && showAdvancedConversion ?
                <div>

                  <div>Power - {sheppardPower}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={sheppardPower}
                    onChange={e => { setSheppardPower(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.sheppard.properties.power.max}
                    min={ConversionMethods.sheppard.properties.power.min}
                  />

                  <div>Nearest - {sheppardNearest}</div>

                  <input
                    className={" mt-2 w-full slider slider-" + accent}
                    value={sheppardNearest}
                    onChange={e => { setSheppardNearest(parseInt(e.target.value)) }}
                    type="range"
                    max={ConversionMethods.sheppard.properties.nearest.max}
                    min={ConversionMethods.sheppard.properties.nearest.min}
                  />
                </div>

                :
                null
            }
          </div>

          <div className="mt-2 p-2 border border-skin-surface0 rounded-xl">
            <div
              className="flex items-center cursor-pointer"
              onClick={() => {
                setConversionMethod(ConversionMethods.nearest_neighbor.codename);
              }}
            >

              <div className="flex-grow font-semibold">{ConversionMethods.nearest_neighbor.name}</div>

              <input
                checked={conversionMethod === ConversionMethods.nearest_neighbor.codename}
                type="radio"
                className={" rounded-full bg-skin-base border border-skin-surface0 " + getRadioColor()}
              />
            </div>
          </div>
        </div>

        <div className="mt-4 bg-skin-mantle p-2 rounded-xl border border-skin-surface0">
          <button
            className="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full disabled:bg-skin-mantle disabled:border-skin-crust flex justify-center items-center w-full"
            onClick={() => { selectImage() }}
            disabled={generatingImages}
          >
            Select Image
          </button>


          {
            selectedImagePath != "" ?
              <div className="">
                <div className="flex justify-center mt-2 mb-2 bg-skin-crust rounded-xl p-2 border border-skin-surface0">
                  <img src={selectedImagePath} className="object-contain h-60 rounded-md"></img>
                </div>

                <button
                  className="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full flex justify-center items-center w-full disabled:bg-skin-mantle disabled:border-skin-crust"
                  onClick={() => { generateImages() }}
                  disabled={generatingImages}
                >
                  Generate Images
                </button>
              </div>
              : null
          }
        </div>


        <div className="flex-grow"></div>

        <div className=" flex justify-start mt-4">
          <button
            className="mr-2 flex items-center bg-skin-mantle p-2 border border-skin-surface0 rounded-xl hover:rounded-full"
            onClick={() => { setShowSettings(true) }}
          >
            <div>
              <SettingsIcon className="h-5 w-5 stroke-skin-text fill-transparent mr-1" />
            </div>
            <div>
              Settings
            </div>
          </button>

          <button
            className="mr-2 flex items-center bg-skin-mantle p-2 border border-skin-surface0 rounded-xl hover:rounded-full"
            onClick={() => { openLink("https://github.com/lighttigerXIV/catppuccinifier") }}
          >
            <div>
              <GitHubIcon className="h-5 w-5 fill-skin-text mr-1" />
            </div>
            <div>
              About
            </div>
          </button>

        </div>
      </div>
    )
  }


  return (
    <div className={" h-screen grid grid-cols-12 min-w-full w-full bg-skin-crust text-skin-text " + theme}>

      <div className=" 2xl:col-span-3 xl:col-span-3 lg:col-span-5 md:col-span-6 2xl:flex xl:flex lg:flex md:flex hidden h-screen max-h-screen overflow-auto">
        <GenerateImageSection />
      </div>


      <div className=" 2xl:hidden xl:hidden lg:hidden md:hidden md-w-60 flex p-4">
        <button className=" hover:bg-skin-surface0 p-1 rounded-full" onClick={() => { setShowSideNav(true) }}>
          <MenuIcon className="w-5 h-5 fill-skin-text" />
        </button>
      </div>

      {
        showSideNav ?
          <div className="h-screen w-screen absolute flex flex-col 2xl:hidden xl:hidden lg:hidden md:hidden md-w-60">
            <div className="w-5/6 bg-skin-crust flex-grow flex flex-col rounded-r-2xl border-r border-t border-b border-skin-surface0 overflow-auto">
              <div className="flex justify-end mt-4 mr-4">
                <button className="hover:bg-skin-surface0 rounded-full p-1 mb-2" onClick={() => { setShowSideNav(false) }}>
                  <CloseIcon className="w-5 h-5 fill-skin-text" />
                </button>
              </div>
              <GenerateImageSection />
            </div>

          </div>
          : null
      }


      <div className="2xl:col-span-9 xl:col-span-9 lg:col-span-7 md:col-span-6  col-span-12 flex overflow-auto">
        <div className="bg-skin-base flex-grow 2xl:rounded-l-2xl xl:rounded-l-2xl lg:rounded-l-2xl md:rounded-l-2xl h-screen overflow-auto ">
          {
            showGeneratedGrids ?

              <div className=" p-3">

                <div className="grid 2xl:grid-cols-3 xl:grid-cols-3 lg:grid-cols-1 md:grid-cols-1 grid-cols-1 gap-2">
                  <div className="">
                    <GeneratedImageBox
                      flavor="latte"
                      imagePath={generatedLattePath}
                      onSaveImage={() => { saveImage("latte") }}
                      onPreviewImage={() => { previewImage("Latte", generatedLatteRawPath) }}
                      disabled={generatingImages}
                    />
                  </div>

                  <div>
                    <GeneratedImageBox
                      flavor="frappe"
                      imagePath={generatedFrappePath}
                      onSaveImage={() => { saveImage("frappe") }}
                      onPreviewImage={() => { previewImage("Frappe", generatedFrappeRawPath) }}
                      disabled={generatingImages}
                    />
                  </div>

                  <div>
                    <GeneratedImageBox
                      flavor="macchiato"
                      imagePath={generatedMacchiatoPath}
                      onSaveImage={() => { saveImage("macchiato") }}
                      onPreviewImage={() => { previewImage("Macchiato", generatedMacchiatoRawPath) }}
                      disabled={generatingImages}
                    />
                  </div>

                  <div>
                    <GeneratedImageBox
                      flavor="mocha"
                      imagePath={generatedMochaPath}
                      onSaveImage={() => { saveImage("mocha") }}
                      onPreviewImage={() => { previewImage("Mocha", generatedMochaRawPath) }}
                      disabled={generatingImages}
                    />
                  </div>

                  <div>
                    <GeneratedImageBox
                      flavor="oled"
                      imagePath={generatedOledPath}
                      onSaveImage={() => { saveImage("oled") }}
                      onPreviewImage={() => { previewImage("Oled", generatedOledRawPath) }}
                      disabled={generatingImages}
                    />
                  </div>

                </div>
              </div>
              : null

          }
        </div>
      </div>

      {
        showSettings ?
          <div className=" absolute h-screen w-screen bg-skin-base flex items-start justify-center max-h-screen overflow-auto">
            <div className="p-4 bg-skin-crust m-4 rounded-2xl">

              <div className="flex justify-end">
                <div
                  className="rounded-full hover:bg-skin-surface0 cursor-pointer"
                  onClick={() => { setShowSettings(false) }}
                >
                  <CloseIcon
                    className="w-5 h-5 fill-skin-text m-1 "

                  />
                </div>
              </div>

              <div className="text-lg">
                Theme
              </div>

              <div className="grid 2xl:grid-cols-4 xl:grid-cols-4 lg:grid-cols-4 md:grid-cols-2 grid-cols-2 gap-2">
                <div
                  className={"cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base " + themesSettingClasses("theme-latte")}
                  onClick={() => {
                    localStorage.setItem("theme", "theme-latte");
                    setTheme("theme-latte");
                  }}
                >
                  <SunflowerIcon className=" w-5 h-5 fill-skin-text" />
                  <div className="ml-2 truncate">
                    Latte
                  </div>
                </div>

                <div
                  className={"cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base " + themesSettingClasses("theme-frappe")}
                  onClick={() => {
                    localStorage.setItem("theme", "theme-frappe");
                    setTheme("theme-frappe");
                  }}
                >
                  <PineappleIcon className=" w-5 h-5 fill-skin-text" />
                  <div className="ml-2 truncate">
                    Frappe
                  </div>
                </div>

                <div
                  className={"cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base " + themesSettingClasses("theme-macchiato")}
                  onClick={() => {
                    localStorage.setItem("theme", "theme-macchiato");
                    setTheme("theme-macchiato");
                  }}
                >
                  <LotusIcon className=" w-5 h-5 fill-skin-text" />
                  <div className="ml-2 truncate">
                    Macchiato
                  </div>
                </div>

                <div
                  className={"cursor-pointer p-2 rounded-xl flex items-center justify-center border hover:bg-skin-base " + themesSettingClasses("theme-mocha")}
                  onClick={() => {
                    localStorage.setItem("theme", "theme-mocha");
                    setTheme("theme-mocha");
                  }}
                >
                  <BranchIcon className=" w-5 h-5 fill-skin-text" />
                  <div className="ml-2 truncate">
                    Mocha
                  </div>
                </div>
              </div>


              <div className="mt-2 text-lg">
                Accent
              </div>

              <div className="grid 2xl:grid-cols-4 xl:grid-cols-4 lg:grid-cols-4 md:grid-cols-4 grid-cols-2 gap-2" >

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("rosewater")}
                  onClick={() => {
                    localStorage.setItem("accent", "rosewater");
                    setAccent("rosewater");
                  }}
                >
                  <div className="text-skin-rosewater ">
                    Rosewater
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("flamingo")}
                  onClick={() => {
                    localStorage.setItem("accent", "flamingo");
                    setAccent("flamingo");
                  }}
                >
                  <div className="text-skin-flamingo">
                    Flamingo
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("pink")}
                  onClick={() => {
                    localStorage.setItem("accent", "pink");
                    setAccent("pink");
                  }}
                >
                  <div className="text-skin-pink">
                    Pink
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("mauve")}
                  onClick={() => {
                    localStorage.setItem("accent", "mauve");
                    setAccent("mauve");
                  }}
                >
                  <div className="text-skin-mauve">
                    Mauve
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("red")}
                  onClick={() => {
                    localStorage.setItem("accent", "red");
                    setAccent("red");
                  }}
                >
                  <div className="text-skin-red">
                    Red
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("maroon")}
                  onClick={() => {
                    localStorage.setItem("accent", "maroon");
                    setAccent("maroon");
                  }}
                >
                  <div className="text-skin-maroon">
                    Maroon
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("peach")}
                  onClick={() => {
                    localStorage.setItem("accent", "peach");
                    setAccent("peach");
                  }}
                >
                  <div className="text-skin-peach">
                    Peach
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("yellow")}
                  onClick={() => {
                    localStorage.setItem("accent", "yellow");
                    setAccent("yellow");
                  }}
                >
                  <div className="text-skin-yellow">
                    Yellow
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("green")}
                  onClick={() => {
                    localStorage.setItem("accent", "green");
                    setAccent("green");
                  }}
                >
                  <div className="text-skin-green">
                    Green
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("teal")}
                  onClick={() => {
                    localStorage.setItem("accent", "teal");
                    setAccent("teal");
                  }}
                >
                  <div className="text-skin-teal">
                    Teal
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("sky")}
                  onClick={() => {
                    localStorage.setItem("accent", "sky");
                    setAccent("sky");
                  }}
                >
                  <div className="text-skin-sky">
                    Sky
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("sapphire")}
                  onClick={() => {
                    localStorage.setItem("accent", "sapphire");
                    setAccent("sapphire");
                  }}
                >
                  <div className="text-skin-sapphire">
                    Sapphire
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("blue")}
                  onClick={() => {
                    localStorage.setItem("accent", "blue");
                    setAccent("blue");
                  }}
                >
                  <div className="text-skin-blue">
                    Blue
                  </div>
                </div>

                <div
                  className={"p-2 rounded-xl border hover:bg-skin-base cursor-pointer flex items-center justify-center " + accentSettingClasses("lavender")}
                  onClick={() => {
                    localStorage.setItem("accent", "lavender");
                    setAccent("lavender");
                  }}
                >
                  <div className="text-skin-lavender">
                    Lavender
                  </div>
                </div>
              </div>
            </div>
          </div>
          : null
      }
    </div>

  );
}

export default App;
