import { convertFileSrc } from "@tauri-apps/api/tauri";

export default function PreviewPage(){

    const params = new URLSearchParams(window.location.search);
    const imagePath = params.get("path");
    const theme = localStorage.getItem("theme") !== null ? localStorage.getItem("theme") : "theme-mocha";

    return(
        <div className={"h-screen min-w-full w-full bg-skin-base text-text p-4 flex justify-center " + theme}>
            <img
                src={convertFileSrc(imagePath)}
                className="object-contain rounded-md h-full"
            >
            </img>
        </div>
    )
}