import { convertFileSrc } from "@tauri-apps/api/tauri";

export default function PreviewPage(){

    const params = new URLSearchParams(window.location.search);
    const imagePath = params.get("path")

    return(
        <div className="h-screen min-w-full w-full bg-base text-text p-4 flex justify-center">
            <img
                src={convertFileSrc(imagePath)}
                className="object-contain rounded-md h-full"
            >
            </img>
        </div>
    )
}