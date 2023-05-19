import { useState, React } from "react";
import "./App.css";

//Tauri
import { open, save } from "@tauri-apps/api/dialog"
import { open as openLink } from '@tauri-apps/api/shell';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { WebviewWindow } from "@tauri-apps/api/window"

//Components
import GeneratedImageBox from "./components/GeneratedImageBox";

//Resources
import { ReactComponent as SettingsIcon } from "./assets/images/settings.svg"
import { ReactComponent as GitHubIcon } from "./assets/images/github.svg"
import { ReactComponent as CloseIcon } from "./assets/images/close.svg"
import { ReactComponent as SunflowerIcon } from "./assets/images/sunflower.svg"
import { ReactComponent as PineappleIcon } from "./assets/images/pineapple.svg"
import { ReactComponent as LotusIcon } from "./assets/images/lotus.svg"
import { ReactComponent as BranchIcon } from "./assets/images/branch.svg"
import { ReactComponent as MenuIcon } from "./assets/images/menu.svg"


function App() {

  const [theme, setTheme] = useState(localStorage.getItem("theme") !== null ? localStorage.getItem("theme") : "theme-mocha")
  const [accent, setAccent] = useState(localStorage.getItem("accent") !== null ? localStorage.getItem("accent") : "blue")
  const [noiseLevel, setNoiseLevel] = useState(4);
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

    await invoke("generate_image", { image_path: selectedImageRawPath, noise_level: noiseLevel.toString(), flavor: "latte" })
      .then(path => {

        setGeneratedLatteRawPath(path);
        setGeneratedLattePath(convertFileSrc(path));
      })
      .catch((error) => { console.error(error) });

    await invoke("generate_image", { image_path: selectedImageRawPath, noise_level: noiseLevel.toString(), flavor: "frappe" })
      .then(path => {

        setGeneratedFrappeRawPath(path);
        setGeneratedFrappePath(convertFileSrc(path));
      })
      .catch((error) => { console.error(error) });

    await invoke("generate_image", { image_path: selectedImageRawPath, noise_level: noiseLevel.toString(), flavor: "macchiato" })
      .then(path => {

        setGeneratedMacchiatoRawPath(path);
        setGeneratedMacchiatoPath(convertFileSrc(path));
      })
      .catch((error) => { console.error(error) });

    await invoke("generate_image", { image_path: selectedImageRawPath, noise_level: noiseLevel.toString(), flavor: "mocha" })
      .then(path => {

        setGeneratedMochaRawPath(path);
        setGeneratedMochaPath(convertFileSrc(path));
      })
      .catch((error) => { console.error(error) });

    await invoke("generate_image", { image_path: selectedImageRawPath, noise_level: noiseLevel.toString(), flavor: "oled" })
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
          fileName = flavor + "-noise" + noiseLevel + "-" + selectedImageRawPath.split('\\').pop();
        } else {
          fileName = flavor + "-noise" + noiseLevel + "-" + selectedImageRawPath.split('/').pop();
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

  //UI

  function GenerateImageSection() {

    return (
      <div className="flex flex-col flex-grow p-4">
        <div className="p-2 bg-skin-mantle rounded-xl border border-skin-surface0">
          <div>Noise Level - {noiseLevel}</div>
          <input className={"w-full slider slider-" + accent} value={noiseLevel} onChange={e => { setNoiseLevel(e.target.value) }} type="range" max={4} min={0} ></input>
        </div>

        <div className="mt-4 bg-skin-mantle p-2 rounded-xl border border-skin-surface0">
          <button
            className="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full disabled:bg-skin-mantle flex justify-center items-center w-full"
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
                  className="bg-skin-crust p-2 rounded-xl border border-skin-surface0 hover:enabled:rounded-full flex justify-center items-center w-full disabled:bg-skin-mantle"
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

      <div className=" 2xl:col-span-3 xl:col-span-3 lg:col-span-5 md:col-span-6 2xl:flex xl:flex lg:flex md:flex hidden h-screen">
        <GenerateImageSection />
      </div>


      <div className=" 2xl:hidden xl:hidden lg:hidden md:hidden md-w-60 flex p-4">
        <button className=" hover:bg-skin-surface0 p-1 rounded-full" onClick={() => { setShowSideNav(true) }}>
          <MenuIcon className="w-5 h-5 fill-skin-text"/>
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
