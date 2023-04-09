$current_dir = $pwd
$files_folder = "$current_dir\installation-files\*"
$catppuccinifier_folder = "C:\Program Files\Catppuccinifier"

Copy-Item -Path $files_folder -Destination $catppuccinifier_folder -Recurse