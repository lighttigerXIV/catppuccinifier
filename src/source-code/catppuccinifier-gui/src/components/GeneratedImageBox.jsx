import { LinearProgress } from "@mui/material"
import { createTheme, ThemeProvider } from "@mui/system"


export default function GeneratedImageBox(props) {

    const muiTheme = createTheme({
        palette: {
            primary: {
                main: "#cba6f7"
            }
        }
    })

    return (
        <div className="p-4 m-2 bg-crust rounded-3xl">
            <div className="flex">
                <button
                    className="bg-surface0 p-2 rounded-xl h-10 flex-grow mr-2  capitalize truncate hover:enabled:rounded-full disabled:bg-surface1"
                    onClick={() => { props.onSaveImage(); }}
                    disabled={props.disabled}
                >
                    Save {props.flavor}
                </button>

                <button
                    className="bg-surface0 p-2 rounded-xl h-10 hover:enabled:rounded-full disabled:bg-surface1"
                    onClick={() => { props.onPreviewImage(); }}
                    disabled={props.disabled}
                >
                    Preview
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
                    <div className="flex mt-4 justify-center  rounded-lg bg-surface0 p-2">
                        <img
                            key={Math.random()}
                            className="rounded-md h-60 object-contain"
                            src={props.imagePath}
                        ></img>
                    </div>
            }
        </div>
    )
}