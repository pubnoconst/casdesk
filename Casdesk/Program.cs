using Avalonia;
using Avalonia.Controls;
using Avalonia.Themes.Fluent;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Media;
using Avalonia.Layout;

namespace Casdesk
{
    public class BaseScene : UserControl
    {
        protected Panel ContentPanel;

        public BaseScene(string title, Window parentWindow)
        {
            var navBar = new DockPanel
            {
                Height = 40,
                LastChildFill = true
            };

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

            ContentPanel = new Panel
            {
                VerticalAlignment = VerticalAlignment.Stretch
            };

            var mainStack = new StackPanel
            {
                VerticalAlignment = VerticalAlignment.Stretch
            };

            mainStack.Children.Add(navBar);
            mainStack.Children.Add(ContentPanel);

            Content = mainStack;
            Padding = new Thickness(12);
        }
    }

    public class FormsScene : BaseScene
    {
        public FormsScene(Window parentWindow) : base("Forms", parentWindow)
        {
            ContentPanel.Children.Add(
                new TextBlock
                {
                    Text = "Forms Content"
                }
            );
        }
    }

    public class QuoteScene : BaseScene
    {
        public QuoteScene(Window parentWindow) : base("Quote", parentWindow)
        {
            ContentPanel.Children.Add(
                new TextBlock
                {
                    Text = "Quote Content"
                }
            );
        }
    }

    public class AdjustScene : BaseScene
    {
        public AdjustScene(Window parentWindow) : base("Adjust", parentWindow)
        {
            ContentPanel.Children.Add(
                new TextBlock
                {
                    Text = "Adjust Content"
                }
            );
        }
    }

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

            var formsBtn = new Button
            {
                Content = new TextBlock
                {
                    Text = "Forms",
                    HorizontalAlignment = HorizontalAlignment.Center,
                    TextAlignment = TextAlignment.Center
                },
                Width = 100
            };
            formsBtn.Click += (_, _) => parentWindow.Content = new FormsScene(parentWindow);

            var quoteBtn = new Button
            {
                Content = new TextBlock
                {
                    Text = "Quote",
                    HorizontalAlignment = HorizontalAlignment.Center,
                    TextAlignment = TextAlignment.Center
                },
                Width = 100
            };
            quoteBtn.Click += (_, _) => parentWindow.Content = new QuoteScene(parentWindow);

            var adjustBtn = new Button
            {

                Content = new TextBlock
                {
                    Text = "Adjust",
                    HorizontalAlignment = HorizontalAlignment.Center,
                    TextAlignment = TextAlignment.Center
                },
                Width = 100
            };
            adjustBtn.Click += (_, _) => parentWindow.Content = new AdjustScene(parentWindow);

            buttonPanel.Children.Add(formsBtn);
            buttonPanel.Children.Add(quoteBtn);
            buttonPanel.Children.Add(adjustBtn);

            stackPanel.Children.Add(textBlock);
            stackPanel.Children.Add(buttonPanel);

            Content = stackPanel;
        }
    }

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

    public class App : Application
    {
        public override void Initialize()
        {
            Styles.Add(new FluentTheme());
        }

        public override void OnFrameworkInitializationCompleted()
        {
            if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                desktop.MainWindow = new MainWindow();
            }

            base.OnFrameworkInitializationCompleted();
        }
    }

    internal class Program
    {
        public static void Main(string[] args)
        {
            BuildAvaloniaApp().StartWithClassicDesktopLifetime(args);
        }

        public static AppBuilder BuildAvaloniaApp()
            => AppBuilder.Configure<App>().UsePlatformDetect().LogToTrace();
    }
}