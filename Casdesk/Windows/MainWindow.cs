using Avalonia.Controls;
using Casdesk.Views;

namespace Casdesk.Windows
{
    public class MainWindow : Window
    {
        public MainWindow()
        {
            Title = "Casdesk 1.2.0";
            Width = 800;
            Height = 550;

            Content = new MainScene(this);
        }
    }
}
