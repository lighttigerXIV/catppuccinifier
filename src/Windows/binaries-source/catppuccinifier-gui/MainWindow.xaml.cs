using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media.Imaging;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;
using Windows.Storage;
using Windows.Storage.Pickers;
using WinRT.Interop;

namespace catppuccinifier
{
    public sealed partial class MainWindow : Window
    {

        string selectedImageName = "";
        string selectedImagePath = "";
        double generatedNoiseLevel = 4;
        private readonly string tempPath = Environment.GetEnvironmentVariable("TEMP") + "\\catppuccinifier";

        public MainWindow()
        {
            InitializeComponent();
            Title = "Catppuccinifier";


            IntPtr hWnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            WindowId myWndId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hWnd);
            var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(myWndId);
            appWindow.SetIcon("Assets/catppuccinifier.ico");



            SelectedImage.Visibility = Visibility.Collapsed;
            GenerateImagesButton.Visibility = Visibility.Collapsed;
            LattePanel.Visibility = Visibility.Collapsed;
            FrappePanel.Visibility = Visibility.Collapsed;
            MacchiatoPanel.Visibility = Visibility.Collapsed;
            MochaPanel.Visibility = Visibility.Collapsed;
            OledPanel.Visibility = Visibility.Collapsed;
        }


        private void NoiseLevelSlider_ValueChanged(object sender, RangeBaseValueChangedEventArgs e)
        {
            NoiseLevelLabel.Text = $"Noise Level - {NoiseLevelSlider.Value}";
        }

        private async void SelectImageButton_Click(object sender, RoutedEventArgs e)
        {
            
            var filePicker = new FileOpenPicker();
            filePicker.FileTypeFilter.Add(".jpg");
            filePicker.FileTypeFilter.Add(".jpeg");
            filePicker.FileTypeFilter.Add(".png");
            filePicker.FileTypeFilter.Add(".webp");


            IntPtr hwnd =WindowNative.GetWindowHandle(this);
            InitializeWithWindow.Initialize(filePicker, hwnd);

            var file = await filePicker.PickSingleFileAsync();

            if(file != null)
            {
                selectedImageName = file.Name;
                selectedImagePath = file.Path;

                BitmapImage bitmapImage = new()
                {
                    UriSource = new Uri(selectedImagePath)
                };

                SelectedImage.Source = bitmapImage;

                SelectedImage.Visibility = Visibility.Visible;
                GenerateImagesButton.Visibility = Visibility.Visible;

                GeneratedLatteImage.Source = null;
                GeneratedFrappeImage.Source = null;
                GeneratedMacchiatoImage.Source = null;
                GeneratedMochaImage.Source = null;
                GeneratedOledImage.Source = null;
            }


            var startInfo = new ProcessStartInfo()
            {
                FileName = "powershell.exe",
                Arguments = $"rm -r  C:\\Windows\\TEMP\\Catppuccinifier",
                CreateNoWindow = true,
                WindowStyle = ProcessWindowStyle.Hidden,
            };

            var process = new Process()
            {
                StartInfo = startInfo,
            };

            process.Start();
        }

        private void GenerateImagesButton_Click(object sender, RoutedEventArgs e)
        {
            try
            {
                DispatcherQueue.TryEnqueue(new Microsoft.UI.Dispatching.DispatcherQueueHandler(async () =>
                {

                    var createTempFolderInfo = new ProcessStartInfo()
                    {
                        FileName = "powershell.exe",
                        Arguments = $"mkdir  C:\\Windows\\TEMP\\Catppuccinifier",
                        CreateNoWindow = true,
                        WindowStyle = ProcessWindowStyle.Hidden,
                    };

                    var createTempFolderProcess = new Process()
                    {
                        StartInfo = createTempFolderInfo,
                    };

                    createTempFolderProcess.Start();

                    generatedNoiseLevel = NoiseLevelSlider.Value;

                    SelectImageButton.IsEnabled = false;
                    GenerateImagesButton.IsEnabled = false;

                    GeneratedLatteImage.Source = null;
                    GeneratedFrappeImage.Source = null;
                    GeneratedMacchiatoImage.Source = null;
                    GeneratedMochaImage.Source = null;
                    GeneratedOledImage.Source = null;


                    LattePanel.Visibility = Visibility.Visible;
                    FrappePanel.Visibility = Visibility.Visible;
                    MacchiatoPanel.Visibility = Visibility.Visible;
                    MochaPanel.Visibility = Visibility.Visible;
                    OledPanel.Visibility = Visibility.Visible;

                    LatteProgressBar.Visibility = Visibility.Visible;
                    FrappeProgressBar.Visibility = Visibility.Visible;
                    MacchiatoProgressBar.Visibility = Visibility.Visible;
                    MochaProgressBar.Visibility = Visibility.Visible;
                    OledProgressBar.Visibility = Visibility.Visible;

                    SaveLatteButton.IsEnabled = false;
                    PreviewLatteButton.IsEnabled = false;

                    SaveFrappeButton.IsEnabled = false;
                    PreviewFrappeButton.IsEnabled = false;

                    SaveMacchiatoButton.IsEnabled = false;
                    PreviewMacchiatoButton.IsEnabled = false;

                    SaveMochaButton.IsEnabled = false;
                    PreviewMochaButton.IsEnabled = false;

                    SaveOledButton.IsEnabled = false;
                    PreviewOledButton.IsEnabled = false;

                    string[] flavors = { "latte", "frappe", "macchiato", "mocha", "oled" };

                    foreach (string flavor in flavors)
                    {
                        string command = $"magick convert '{selectedImagePath}' 'C:\\Program Files\\Catppuccinifier\\flavors\\noise-{generatedNoiseLevel}\\{flavor}.png' -hald-clut '{tempPath}\\{flavor}.png'";

                        var startInfo = new ProcessStartInfo()
                        {
                            FileName = "powershell.exe",
                            Arguments = command,
                            CreateNoWindow = true,
                            WindowStyle = ProcessWindowStyle.Hidden,
                        };

                        var process = new Process()
                        {
                            StartInfo = startInfo,
                        };

                        await Task.Run(() => {

                            process.Start();
                            process.WaitForExit();
                        });

                        BitmapImage bitmapImage = new()
                        {
                            CreateOptions = BitmapCreateOptions.IgnoreImageCache
                        };

                        if (flavor == "latte")
                        {

                            bitmapImage.UriSource = new Uri($"{tempPath}\\latte.png");
                            GeneratedLatteImage.Source = bitmapImage;
                            LatteProgressBar.Visibility = Visibility.Collapsed;
                        }

                        if (flavor == "frappe")
                        {
                            bitmapImage.UriSource = new Uri($"{tempPath}\\frappe.png");
                            GeneratedFrappeImage.Source = bitmapImage;
                            FrappeProgressBar.Visibility = Visibility.Collapsed;
                        }

                        if (flavor == "macchiato")
                        {
                            bitmapImage.UriSource = new Uri($"{tempPath}\\macchiato.png");
                            GeneratedMacchiatoImage.Source = bitmapImage;
                            MacchiatoProgressBar.Visibility = Visibility.Collapsed;
                        }

                        if (flavor == "mocha")
                        {
                            bitmapImage.UriSource = new Uri($"{tempPath}\\mocha.png");
                            GeneratedMochaImage.Source = bitmapImage;
                            MochaProgressBar.Visibility = Visibility.Collapsed;
                        }

                        if (flavor == "oled")
                        {
                            bitmapImage.UriSource = new Uri($"{tempPath}\\oled.png");
                            GeneratedOledImage.Source = bitmapImage;
                            OledProgressBar.Visibility = Visibility.Collapsed;
                        }
                    }


                    SelectImageButton.IsEnabled = true;
                    GenerateImagesButton.IsEnabled = true;

                    SaveLatteButton.IsEnabled = true;
                    PreviewLatteButton.IsEnabled = true;

                    SaveFrappeButton.IsEnabled = true;
                    PreviewFrappeButton.IsEnabled = true;

                    SaveMacchiatoButton.IsEnabled = true;
                    PreviewMacchiatoButton.IsEnabled = true;

                    SaveMochaButton.IsEnabled = true;
                    PreviewMochaButton.IsEnabled = true;

                    SaveOledButton.IsEnabled = true;
                    PreviewOledButton.IsEnabled = true;
                }));
            }
             catch(Exception exc) { Debug.WriteLine(exc); }
        }

        private void SaveLatteButton_Click(object sender, RoutedEventArgs e)
        {
            SaveImage("latte");
        }

        private void SaveFrappeButton_Click(object sender, RoutedEventArgs e)
        {
            SaveImage("frappe");
        }

        private void SaveMacchiatoButton_Click(object sender, RoutedEventArgs e)
        {
            SaveImage("macchiato");
        }

        private void SaveMochaButton_Click(object sender, RoutedEventArgs e)
        {
            SaveImage("mocha");
        }

        private void SaveOledButton_Click(object sender, RoutedEventArgs e)
        {
            SaveImage("oled");
        }

        private void PreviewLatteButton_Click(object sender, RoutedEventArgs e)
        {
            ShowPreview("latte");
        }

        private void PreviewFrappeButton_Click(object sender, RoutedEventArgs e)
        {
            ShowPreview("frappe");
        }

        private void PreviewMacchiatoButton_Click(object sender, RoutedEventArgs e)
        {
            ShowPreview("macchiato");
        }

        private void PreviewMochaButton_Click(object sender, RoutedEventArgs e)
        {
            ShowPreview("mocha");
        }

        private void PreviewOledButton_Click(object sender, RoutedEventArgs e)
        {
            ShowPreview("oled");
        }


        private async void SaveImage(string flavor)
        {
            var fileSaver = new FileSavePicker
            {
                SuggestedStartLocation = PickerLocationId.PicturesLibrary
            };
            fileSaver.FileTypeChoices.Add("Png", new List<string>() { ".jpg", ".jpeg", ".png", ".webp" });
            fileSaver.SuggestedFileName = $"{flavor}-noise-{generatedNoiseLevel}-{selectedImageName}";

            IntPtr hwnd = WindowNative.GetWindowHandle(this);
            InitializeWithWindow.Initialize(fileSaver, hwnd);

            var file = await fileSaver.PickSaveFileAsync();

            if(file != null)
            {

                var startInfo = new ProcessStartInfo()
                {
                    FileName = "powershell.exe",
                    Arguments = $"cp {tempPath}\\{flavor}.png {file.Path}",
                    CreateNoWindow = true,
                    WindowStyle = ProcessWindowStyle.Hidden,
                };

                Debug.WriteLine($"cp {tempPath}\\{flavor}.png {file.Path}");

                var process = new Process()
                {
                    StartInfo = startInfo,
                };

                process.Start();
            }
        }

        private void ShowPreview(string flavor)
        {
            PreviewWindow previewWindow = new($"{tempPath}\\{flavor}.png");
            previewWindow.Activate();
        }
    }
}
