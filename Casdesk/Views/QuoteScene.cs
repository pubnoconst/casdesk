using Avalonia.Controls;

namespace Casdesk.Views
{
    public class QuoteScene : BaseScene
    {
        public QuoteScene(Window parentWindow) : base("Quote", parentWindow)
        {
            ContentPanel.Children.Add(new TextBlock { Text = "Quote Content" });
        }
    }
}
