using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace Catppuccinifier
{
    public partial class PreviewImageForm : Form
    {
        public String imagePath = "";

        public PreviewImageForm()
        {
            InitializeComponent();
        }

        private void PreviewImageForm_Load(object sender, EventArgs e)
        {

            PreviewBox.ImageLocation = imagePath;
        }
    }
}
