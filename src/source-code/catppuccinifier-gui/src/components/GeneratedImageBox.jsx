import { LinearProgress } from "@mui/material"
import { createTheme, ThemeProvider } from "@mui/system"

//Resources
import { ReactComponent as SaveIcon } from "../assets/images/save.svg"
import { ReactComponent as MaximizeIcon } from "../assets/images/maximize.svg"

function getAccentHexColor() {

    const theme = localStorage.getItem("theme") !== null ? localStorage.getItem("theme") : "theme-mocha";
    const accent = localStorage.getItem("accent") !== null ? localStorage.getItem("accent") : "blue";

    switch(theme){
        case "theme-mocha": 
            switch(accent){
                case "rosewater": return "#f5e0dc"
                case "flamingo": return "#f2cdcd"
                case "pink": return "#f5c2e7"
                case "mauve": return "#cba6f7"
                case "red": return "#f38ba8"
                case "maroon": return "#eba0ac"
                case "peach": return "#fab387"
                case "yellow": return "#f9e2af"
                case "green": return "#a6e3a1"
                case "teal": return "#94e2d5"
                case "sky": return "#89dceb"
                case "sapphire": return "#74c7ec"
                case "blue": return "#89b4fa"
                case "lavender": return "#b4befe"
            }
        
        case "theme-macchiato": 
            switch(accent){
                case "rosewater": return "#f4dbd6"
                case "flamingo": return "#f0c6c6"
                case "pink": return "#f5bde6"
                case "mauve": return "#c6a0f6"
                case "red": return "#ed8796"
                case "maroon": return "#ee99a0"
                case "peach": return "#f5a97f"
                case "yellow": return "#eed49f"
                case "green": return "#a6da95"
                case "teal": return "#8bd5ca"
                case "sky": return "#91d7e3"
                case "sapphire": return "#7dc4e4"
                case "blue": return "#8aadf4"
                case "lavender": return "#b7bdf8"
            }

        case "theme-frappe": 
            switch(accent){
                case "rosewater": return "#f2d5cf"
                case "flamingo": return "#eebebe"
                case "pink": return "#f4b8e4"
                case "mauve": return "#ca9ee6"
                case "red": return "#e78284"
                case "maroon": return "#ea999c"
                case "peach": return "#ef9f76"
                case "yellow": return "#e5c890"
                case "green": return "#a6d189"
                case "teal": return "#81c8be"
                case "sky": return "#99d1db"
                case "sapphire": return "#85c1dc"
                case "blue": return "#8caaee"
                case "lavender": return "#babbf1"
            }

        case "theme-latte": 
            switch(accent){
                case "rosewater": return "#dc8a78"
                case "flamingo": return "#dd7878"
                case "pink": return "#ea76cb"
                case "mauve": return "#8839ef"
                case "red": return "#d20f39"
                case "maroon": return "#e64553"
                case "peach": return "#fe640b"
                case "yellow": return "#df8e1d"
                case "green": return "#40a02b"
                case "teal": return "#179299"
                case "sky": return "#04a5e5"
                case "sapphire": return "#209fb5"
                case "blue": return "#1e66f5"
                case "lavender": return "#7287fd"
            }
    }
}


export default function GeneratedImageBox(props) {

    const muiTheme = createTheme({
        palette: {
            primary: {
                main: getAccentHexColor()
            }
        }
    })

    return (
        <div className="p-4 bg-skin-crust border border-skin-surface0 rounded-3xl aspect-square flex flex-col">
            <div className="flex">
                <button
                    className="bg-skin-mantle border border-skin-surface0 p-2 rounded-xl h-10 flex-grow mr-2 capitalize truncate hover:enabled:rounded-full disabled:bg-skin-crust disabled:border-skin-mantle"
                    onClick={() => { props.onSaveImage(); }}
                    disabled={props.disabled}
                >
                    <div className="flex items-center justify-center">
                        <div>
                            <SaveIcon className="mr-2 h-5 w-5 stroke-skin-text fill-transparent"/>
                        </div>
                        <div>
                            Save {props.flavor}
                        </div>
                        
                    </div>
                </button>

                <button
                    className="bg-skin-mantle border border-skin-surface0 p-2 rounded-xl h-10 hover:enabled:rounded-full disabled:bg-skin-crust disabled:border-skin-mantle"
                    onClick={() => { props.onPreviewImage(); }}
                    disabled={props.disabled}
                >
                    <div className="flex items-center justify-center">
                        <div>
                            <MaximizeIcon className="mr-2 h-5 w-5 stroke-skin-text fill-transparent"/>
                        </div>
                        <div>
                            Preview
                        </div>
                    </div>
                </button>
            </div>

            {
                props.imagePath.length === 0 ?
                    <div>
                        <ThemeProvider theme={muiTheme}>
                            <LinearProgress className="mt-4" />
                        </ThemeProvider>
                    </div>
                    :
                    <div className="flex flex-grow mt-4 justify-center  rounded-lg bg-skin-mantle border border-skin-surface0 p-2">
                        <img
                            key={Math.random()}
                            className="rounded-md  object-contain"
                            src={props.imagePath}
                        ></img>
                    </div>
            }
        </div>
    )
}