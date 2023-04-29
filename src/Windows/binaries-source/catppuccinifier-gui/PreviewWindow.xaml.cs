using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Data;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Microsoft.UI.Xaml.Navigation;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices.WindowsRuntime;
using Windows.Foundation;
using Windows.Foundation.Collections;


namespace catppuccinifier
{
    public sealed partial class PreviewWindow : Window
    {
        public PreviewWindow(string imagePath)
        {
            InitializeComponent();
            Title = "Preview";

            IntPtr hWnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            WindowId myWndId = Win32Interop.GetWindowIdFromWindow(hWnd);
            var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(myWndId);
            appWindow.SetIcon("Assets/catppuccinifier.ico");


            try
            {
                var bitmap = new BitmapImage()
                {
                    CreateOptions = BitmapCreateOptions.IgnoreImageCache,
                    UriSource = new Uri(imagePath)
                };

                PreviewImage.Source = bitmap;
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex);
            }
        }
    }
}
