import { useState, React } from "react";
import "./App.css";

//Tauri
import { open, save } from "@tauri-apps/api/dialog"
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { open as openLink } from "@tauri-apps/api/shell"
import { WebviewWindow } from "@tauri-apps/api/window"

//Components
import { Slider, SvgIcon } from "@mui/material";
import GeneratedImageBox from "./components/GeneratedImageBox";


function App() {

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

    const fileName = flavor + "-noise" + noiseLevel + "-" + selectedImageRawPath.split('/').pop()

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


  return (
    <div className="grid grid-cols-12 h-full min-h-screen min-w-full w-full bg-base text-text">
      <div className="xl:col-span-3 lg:col-span-4 h-full md:col-span-5 col-span-12 bg-crust p-3 flex flex-col">
        <div className="">
          <div className="text-2xl">Properties</div>
          <div>Noise Level - {noiseLevel}</div>
          <Slider size="small" defaultValue={4} max={4} color="secondary" onChange={e => { setNoiseLevel(e.target.value) }}></Slider>



          <button
            className="bg-surface0 p-2 rounded-xl hover:enabled:rounded-full disabled:bg-surface1 flex justify-center items-center w-full"
            onClick={() => { selectImage() }}
            disabled={generatingImages}
          >
            Select Image
          </button>


          {
            selectedImagePath != "" ?
              <div>

                <div className="w-100 flex justify-center my-4 bg-surface0 p-2 rounded-lg">
                  <img src={selectedImagePath} className="object-contain h-60 rounded-md"></img>
                </div>

                <button
                  className="bg-surface0 p-2 rounded-xl hover:enabled:rounded-full flex justify-center items-center w-full disabled:bg-surface1"
                  onClick={() => { generateImages() }}
                  disabled={generatingImages}
                >
                  Generate Images
                </button>
              </div>
              : null
          }
        </div>

        <div className="flex-1 flex items-end ">
          <div className="flex items-center">
            <div
              className="mr-2 mt-2"
            >
              Follow the project on
              <b
                className="text-mauve hover:cursor-pointer ml-1"
                onClick={() => { openLink("https://github.com/lighttigerXIV/catppuccinifier") }}
              >
                GitHub
              </b>
            </div>
          </div>
        </div>
      </div>

      {
        showGeneratedGrids ?

          <div className="xl:col-span-9 lg:col-span-8 md:col-span-7 col-span-12 p-3">

            <div className="text-2xl">Generated Images</div>

            <div className="grid xl:grid-cols-3 lg:grid-cols-2 md:grid-cols-1">
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

  );
}

export default App;
