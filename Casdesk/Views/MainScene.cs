using Avalonia.Controls;
using Avalonia.Layout;

namespace Casdesk.Views
{
    public class MainScene : UserControl
    {
        public MainScene(Window parentWindow)
        {
            var stackPanel = new StackPanel
            {
                HorizontalAlignment = HorizontalAlignment.Center,
                VerticalAlignment = VerticalAlignment.Center,
                Spacing = 20
            };

            var textBlock = new TextBlock
            {
                Text = "Casdesk",
                HorizontalAlignment = HorizontalAlignment.Center,
                FontSize = 64
            };

            var buttonPanel = new StackPanel
            {
                Orientation = Orientation.Horizontal,
                HorizontalAlignment = HorizontalAlignment.Center,
                Spacing = 10
            };

            var formsBtn = new Button { Content = new TextBlock { Text = "Forms", HorizontalAlignment = HorizontalAlignment.Center }, Width = 100 };
            formsBtn.Click += (_, _) => parentWindow.Content = new FormsScene(parentWindow);

            var quoteBtn = new Button { Content = new TextBlock { Text = "Quote", HorizontalAlignment = HorizontalAlignment.Center }, Width = 100 };
            quoteBtn.Click += (_, _) => parentWindow.Content = new QuoteScene(parentWindow);

            var adjustBtn = new Button { Content = new TextBlock { Text = "Adjust", HorizontalAlignment = HorizontalAlignment.Center }, Width = 100 };
            adjustBtn.Click += (_, _) => parentWindow.Content = new AdjustScene(parentWindow);

            buttonPanel.Children.Add(formsBtn);
            buttonPanel.Children.Add(quoteBtn);
            buttonPanel.Children.Add(adjustBtn);

            stackPanel.Children.Add(textBlock);
            stackPanel.Children.Add(buttonPanel);

            Content = stackPanel;
        }
    }
}
