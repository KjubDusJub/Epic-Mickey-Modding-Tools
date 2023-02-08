using System.Diagnostics;
using VGAudio.Containers.Dsp;
using VGAudio.Containers.Wave;
using VGAudio.Formats;

namespace AudioModder
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }


      

        private void Form1_Load(object sender, EventArgs e)
        {

        }

        public void AutomateDSPCreation(string sourceFile)
        {
            using (var fs = new StreamReader(sourceFile))
            {
                AudioData data = new WaveReader().Read(fs.BaseStream);

                byte[] final = new DspWriter().GetFile(data);

                File.WriteAllBytes("final.dsp", final);
            }
        }

        private void convert_Click(object sender, EventArgs e)
        {
            OpenFileDialog src = new OpenFileDialog();

            src.Filter = "DSP File (*.dsp)|*.dsp|WAV File (*.wav)|*.wav";

            if (src.ShowDialog() == DialogResult.OK)
            {
                string path = src.FileName;
                if(src.FileName.EndsWith(".wav"))
                {
                    AutomateDSPCreation(path);
                    path = "final.dsp";
                }

                var process = new Process
                {
                    StartInfo =
              {
                  FileName = Directory.GetCurrentDirectory() + "/Files/audiomodder.exe",
                  UseShellExecute = true,
                  Arguments = "\"" + path + "\"  ",
                  CreateNoWindow = false
              }
                };
                process.Start();

                process.WaitForExit();


                int i = process.ExitCode;
                if (i == 0)
                {
                    SaveFileDialog dest = new SaveFileDialog();

                    dest.Filter = "WEM File (*.wem)|*.wem";

                    if (dest.ShowDialog() == DialogResult.OK)
                    {
                        File.Move("temp", dest.FileName);
                    }

                    label1.Text = "Audio Modder Process ended successfully!";
                    MessageBox.Show("Successfully converted file!");
                }
                else
                {
                    MessageBox.Show("Oops!! The audio modder has run into an error! (" + i + ")");
                    label1.Text = "Hmm.. It seems that the Audio Modder Process might've thrown an error... (Error Code: " + i + ")";
                }






            }
        }

        private void button2_Click(object sender, EventArgs e)
        {
            MessageBox.Show("Created by memer#1024. If you're gonna use this tool for a mod feel free to credit me i want attention");
        }
       

        private void button1_Click_1(object sender, EventArgs e)
        {
            Form2 f = new Form2();
            f.ShowDialog();
        }
    }
}