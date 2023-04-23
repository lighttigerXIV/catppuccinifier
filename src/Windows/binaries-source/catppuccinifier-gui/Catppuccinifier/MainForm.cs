using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using static System.Windows.Forms.VisualStyles.VisualStyleElement;

namespace Catppuccinifier
{
    public partial class Catppuccinifier : Form
    {

        private String selectedImagePath = "";
        private String selectedImageName = "";
        private readonly String tempPath = Environment.GetEnvironmentVariable("TEMP") + "\\catppuccinifier";

        private void OpenPreview(String flavor)
        {
            PreviewImageForm previewForm = new PreviewImageForm
            {
                imagePath = $"{tempPath}\\{flavor}.png"
            };

            previewForm.Show();
        }

        private void SaveImage(String flavor)
        {
            SaveFileDialog dialog = new SaveFileDialog
            {
                FileName = $"{flavor}-{selectedImageName}",
                Filter = "Images|*.jpg;*.jpeg;*.png"
            };

            if (dialog.ShowDialog() == DialogResult.OK)
            {

                String savePath = dialog.FileName;

                Process process = new Process();
                process.StartInfo.FileName = "powershell.exe";
                process.StartInfo.Arguments = $"cp '{tempPath}\\{flavor}.png' '{savePath}'";
                process.StartInfo.WindowStyle = ProcessWindowStyle.Hidden;
                process.Start();
            }
        }

        public Catppuccinifier()
        {
            InitializeComponent();
            SelectedImageBox.Visible = false;
            GenerateImagesButton.Visible = false;
            GeneratedLattePanel.Visible = false;
            GeneratedFrappePanel.Visible = false;
            GeneratedMacchiatoPanel.Visible = false;
            GeneratedMochaPanel.Visible = false;
            GeneratedOledPanel.Visible = false;

            //Creates temp folder
            Process process = new Process();
            process.StartInfo.FileName = "powershell.exe";
            process.StartInfo.Arguments = $"mkdir -P {tempPath}";
            process.StartInfo.WindowStyle = ProcessWindowStyle.Hidden;
            process.Start();
        }

        private void NoiseLevelTrackbar_ValueChanged(object sender, EventArgs e)
        {
            CurrentNoiseLevelLabel.Text = NoiseLevelTrackbar.Value.ToString();
        }

        private void SelectImageButton_Click(object sender, EventArgs e)
        {
            OpenFileDialog dialog = new OpenFileDialog
            {
                Title = "Select Image",
                Filter = "Images|*.jpg;*.jpeg;*.png"
            };

            dialog.ShowDialog();

            if(dialog.FileName != "")
            {
                selectedImagePath = dialog.FileName;
                selectedImageName = Path.GetFileName(dialog.FileName);
                SelectedImageBox.ImageLocation = dialog.FileName;
                SelectedImageBox.Visible = true;
                GenerateImagesButton.Visible = true;
                GeneratedLatteBox.Image = null;
                GeneratedFrappeBox.Image = null;
                GeneratedMacchiatoBox.Image = null;
                GeneratedMochaBox.Image = null;
                GeneratedOledBox.Image = null;
            }
        }

        private async void GenerateImagesButton_Click(object sender, EventArgs e)
        {
            SelectImageButton.Enabled = false;
            GenerateImagesButton.Enabled = false;

            GeneratedLatteBox.Image = null;
            GeneratedFrappeBox.Image = null;
            GeneratedMacchiatoBox.Image = null;
            GeneratedMochaBox.Image = null;
            GeneratedOledBox.Image = null;

            GeneratedLattePanel.Visible = true;
            GeneratedFrappePanel.Visible = true;
            GeneratedMacchiatoPanel.Visible = true;
            GeneratedMochaPanel.Visible = true;
            GeneratedOledPanel.Visible = true;

            SaveLatteButton.Enabled = false;
            PreviewLatteButton.Enabled = false;

            SaveFrappeButton.Enabled = false;
            PreviewFrappeButton.Enabled = false;

            SaveMacchiatoButton.Enabled = false;
            PreviewMacchiatoButton.Enabled = false;

            SaveMochaButton.Enabled = false;
            PreviewMochaButton.Enabled = false;
            
            SaveOledButton.Enabled = false;
            PreviewOledButton.Enabled = false;


            String[] flavors = { "latte", "frappe", "macchiato", "mocha", "oled" };

            await Task.Run(() =>
            {

                foreach (String flavor in flavors)
                {
                    String command = $"magick convert '{selectedImagePath}' 'C:\\Program Files\\Catppuccinifier\\flavors\\noise-{CurrentNoiseLevelLabel.Text}\\{flavor}.png' -hald-clut '{tempPath}\\{flavor}.png'";

                    Process process = new Process();
                    process.StartInfo.FileName = "powershell.exe";
                    process.StartInfo.Arguments = command;
                    process.StartInfo.WindowStyle = ProcessWindowStyle.Hidden;
                    process.Start();
                    process.WaitForExit();

                    if (flavor == "latte")
                    {
                        GeneratedLatteBox.ImageLocation = $"{tempPath}\\latte.png";
                    }

                    if (flavor == "frappe")
                    {
                        GeneratedFrappeBox.ImageLocation = $"{tempPath}\\frappe.png";
                    }

                    if (flavor == "macchiato")
                    {
                        GeneratedMacchiatoBox.ImageLocation = $"{tempPath}\\macchiato.png";
                    }

                    if (flavor == "mocha")
                    {
                        GeneratedMochaBox.ImageLocation = $"{tempPath}\\mocha.png";
                    }

                    if (flavor == "oled")
                    {
                        GeneratedOledBox.ImageLocation = $"{tempPath}\\oled.png";
                    }
                }
            });

            SelectImageButton.Enabled = true;
            GenerateImagesButton.Enabled = true;

            SaveLatteButton.Enabled = true;
            PreviewLatteButton.Enabled = true;

            SaveFrappeButton.Enabled = true;
            PreviewFrappeButton.Enabled = true;

            SaveMacchiatoButton.Enabled = true;
            PreviewMacchiatoButton.Enabled = true;

            SaveMochaButton.Enabled = true;
            PreviewMochaButton.Enabled = true;

            SaveOledButton.Enabled = true;
            PreviewOledButton.Enabled = true;
        }

        private void PreviewLatteButton_Click(object sender, EventArgs e)
        {
            OpenPreview("latte");
        }

        

        private void SaveLatteButton_Click(object sender, EventArgs e)
        {
            SaveImage("latte");   
        }

        private void SaveMacchiatoButton_Click(object sender, EventArgs e)
        {
            SaveImage("macchiato");
        }

        private void PreviewMacchiatoButton_Click(object sender, EventArgs e)
        {
            OpenPreview("macchiato");
        }

        private void SaveOledButton_Click(object sender, EventArgs e)
        {
            SaveImage("oled");
        }

        private void PreviewOledButton_Click(object sender, EventArgs e)
        {
            OpenPreview("oled");
        }

        private void SaveFrappeButton_Click(object sender, EventArgs e)
        {
            SaveImage("frappe");
        }

        private void PreviewFrappeButton_Click(object sender, EventArgs e)
        {
            OpenPreview("frappe");
        }

        private void SaveMochaButton_Click(object sender, EventArgs e)
        {
            SaveImage("mocha");
        }

        private void PreviewMochaButton_Click(object sender, EventArgs e)
        {
            OpenPreview("mocha");
        }
    }
}
