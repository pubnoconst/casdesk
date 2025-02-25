using Avalonia.Controls;
using Avalonia.Layout;
using Avalonia.Media;

namespace Casdesk.Views
{
    public class BaseScene : UserControl
    {
        protected Panel ContentPanel;

        public BaseScene(string title, Window parentWindow)
        {
            var navBar = new DockPanel { Height = 40, LastChildFill = true };

            var backButton = new Button
            {
                Content = new TextBlock
                {
                    Text = "Back",
                    HorizontalAlignment = HorizontalAlignment.Center,
                    TextAlignment = TextAlignment.Center
                },
                Width = 75
            };
            backButton.Click += (_, _) => parentWindow.Content = new MainScene(parentWindow);

            var spacer = new StackPanel { HorizontalAlignment = HorizontalAlignment.Stretch };

            var titleText = new TextBlock
            {
                Text = title,
                HorizontalAlignment = HorizontalAlignment.Right,
                VerticalAlignment = VerticalAlignment.Center,
                FontSize = 24
            };

            DockPanel.SetDock(backButton, Dock.Left);
            DockPanel.SetDock(titleText, Dock.Right);

            navBar.Children.Add(backButton);
            navBar.Children.Add(spacer);
            navBar.Children.Add(titleText);

            ContentPanel = new Panel { VerticalAlignment = VerticalAlignment.Stretch };

            var mainStack = new StackPanel { VerticalAlignment = VerticalAlignment.Stretch };
            mainStack.Children.Add(navBar);
            mainStack.Children.Add(ContentPanel);

            Content = mainStack;
            Padding = new Avalonia.Thickness(12);
        }
    }
}
