using Avalonia.Controls;

namespace Casdesk.Views
{
    public class AdjustScene : BaseScene
    {
        public AdjustScene(Window parentWindow) : base("Adjust", parentWindow)
        {
            ContentPanel.Children.Add(new TextBlock { Text = "Adjust Content" });
        }
    }
}
