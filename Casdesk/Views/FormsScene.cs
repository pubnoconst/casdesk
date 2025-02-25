using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.Primitives;
using Avalonia.Layout;
using Avalonia.Media;
using System.Collections.Generic;

namespace Casdesk.Views
{
    
    // ─────────────────────────────────────────────
    // Base class for form-based content (UserControl)
    // ─────────────────────────────────────────────
    public abstract class FormTab : UserControl
    {
        protected Grid FormGrid { get; }
        protected Button SubmitButton { get; }

        public FormTab()
        {
            FormGrid = new Grid
            {
                Margin = new Thickness(10),
                ColumnDefinitions = new ColumnDefinitions("200,350")
            };

            SubmitButton = new Button
            {
                Content = new TextBlock { Text = "Confirm", HorizontalAlignment = HorizontalAlignment.Center },
                HorizontalAlignment = HorizontalAlignment.Center,
                Width = 300,
                Margin = new Thickness(0, 10, 0, 0)
            };

            BuildForm();

            // Add submit button
            var submitRow = FormGrid.RowDefinitions.Count;
            FormGrid.RowDefinitions.Add(new RowDefinition(GridLength.Auto));
            Grid.SetColumnSpan(SubmitButton, 2);
            Grid.SetRow(SubmitButton, submitRow);
            FormGrid.Children.Add(SubmitButton);

            // Centered container with scroll
            var scrollViewer = new ScrollViewer
            {
                Content = new StackPanel
                {
                    HorizontalAlignment = HorizontalAlignment.Center,
                    Children = { FormGrid }
                },
                HorizontalScrollBarVisibility = ScrollBarVisibility.Auto,
                VerticalScrollBarVisibility = ScrollBarVisibility.Auto
            };

            Content = scrollViewer;
        }

        protected abstract void BuildForm();

        protected void AddRow(int row, string labelText, Control control)
        {
            while (FormGrid.RowDefinitions.Count <= row)
                FormGrid.RowDefinitions.Add(new RowDefinition(GridLength.Auto));

            var label = new TextBlock
            {
                Text = labelText,
                VerticalAlignment = VerticalAlignment.Center,
                Margin = new Thickness(0, 0, 10, 0)
            };

            Grid.SetColumn(label, 0);
            Grid.SetRow(label, row);
            FormGrid.Children.Add(label);

            Grid.SetColumn(control, 1);
            Grid.SetRow(control, row);
            FormGrid.Children.Add(control);
        }
    }

    // ─────────────────────────────────────────────
    // Device Sale Base
    // ─────────────────────────────────────────────
    public abstract class DeviceSaleTab : FormTab
    {
        protected TextBox? CustomerNameField;
        protected TextBox? DeviceField;
        protected TextBox? DeviceColorField;
        protected TextBox? DeviceImeiField;
        protected TextBox? DeviceProviderField;
        protected TextBox? DevicePriceField;
        protected TextBox? PaymentMethodField;
        protected TextBox? CustomerContactField;
        protected TextBox? CustomerAddressField;
        protected TextBox? CustomerIdField;
        protected TextBox? StaffNameField;
        protected DatePicker? DateField;

        protected override void BuildForm()
        {
            int row = 0;
            CustomerNameField = new TextBox { Watermark = "Name" };
            AddRow(row++, "Customer Name", CustomerNameField);

            DeviceField = new TextBox { Watermark = "Device name and model" };
            AddRow(row++, "Device", DeviceField);

            DeviceColorField = new TextBox { Watermark = "Blue/Red/Purple..." };
            AddRow(row++, "Device Color", DeviceColorField);

            DeviceImeiField = new TextBox { Watermark = "*#06#" };
            AddRow(row++, "Device IMEI", DeviceImeiField);

            DeviceProviderField = new TextBox { Watermark = "Unlocked/Optus..." };
            AddRow(row++, "Device Provider", DeviceProviderField);

            DevicePriceField = new TextBox { Watermark = "Exact amount in AUD" };
            AddRow(row++, "Device Price", DevicePriceField);

            PaymentMethodField = new TextBox { Watermark = "EFTPOS/Cash/Stripe" };
            AddRow(row++, "Payment Method", PaymentMethodField);

            CustomerContactField = new TextBox { Watermark = "04XY XYZ XYZ" };
            AddRow(row++, "Customer Contact", CustomerContactField);

            CustomerAddressField = new TextBox { Watermark = "1/23 John Whiteway Dr, Nth Avoca, NSW 2257" };
            AddRow(row++, "Customer Address", CustomerAddressField);

            CustomerIdField = new TextBox { Watermark = "2323452" };
            AddRow(row++, "Customer ID", CustomerIdField);

            StaffNameField = new TextBox { Watermark = "Your name" };
            AddRow(row++, "Staff Name", StaffNameField);

            DateField = new DatePicker()
            {
                HorizontalAlignment = HorizontalAlignment.Stretch
            };
            AddRow(row++, "Date", DateField);
        }
    }

    public class RefurbishedDeviceSaleTab : DeviceSaleTab { }
    public class NewDeviceSaleTab : DeviceSaleTab { }

    // ─────────────────────────────────────────────
    // Device Purchase Form
    // ─────────────────────────────────────────────
    public class DevicePurchaseTab : FormTab
    {
        private TextBox? SellerNameField;
        private TextBox? DeviceField;
        private TextBox? DeviceColorField;
        private TextBox? MemoryField;
        private TextBox? ImeiField;
        private TextBox? DeviceProviderField;
        private TextBox? PurchasePriceField;
        private TextBox? SellerContactField;
        private TextBox? SellerAddressField;
        private TextBox? SellerIdField;
        private TextBox? StaffNameField;
        private DatePicker? DateField;
        private TextBox? NoteField;

        protected override void BuildForm()
        {
            int row = 0;
            SellerNameField = new TextBox { Watermark = "Name" };
            AddRow(row++, "Seller's Name", SellerNameField);

            DeviceField = new TextBox { Watermark = "Device name and model" };
            AddRow(row++, "Device", DeviceField);

            DeviceColorField = new TextBox { Watermark = "Blue/Red/Purple" };
            AddRow(row++, "Device Color", DeviceColorField);

            MemoryField = new TextBox { Watermark = "In GB" };
            AddRow(row++, "Memory", MemoryField);

            ImeiField = new TextBox { Watermark = "*#06#" };
            AddRow(row++, "IMEI", ImeiField);

            DeviceProviderField = new TextBox { Watermark = "Unlocked/Optus..." };
            AddRow(row++, "Device Provider", DeviceProviderField);

            PurchasePriceField = new TextBox { Watermark = "Exact amount in AUD" };
            AddRow(row++, "Purchase Price", PurchasePriceField);

            SellerContactField = new TextBox { Watermark = "04XY XYZ XYZ" };
            AddRow(row++, "Seller's Contact", SellerContactField);

            SellerAddressField = new TextBox { Watermark = "1/23 John Whiteway Dr, Nth Avoca, NSW 2257" };
            AddRow(row++, "Seller's Address", SellerAddressField);

            SellerIdField = new TextBox { Watermark = "2323452" };
            AddRow(row++, "Seller's ID", SellerIdField);

            StaffNameField = new TextBox { Watermark = "Your name" };
            AddRow(row++, "Staff Name", StaffNameField);

            DateField = new DatePicker()
            {
                HorizontalAlignment = HorizontalAlignment.Stretch
            };
            AddRow(row++, "Date", DateField);

            NoteField = new TextBox { Watermark = "Any notes/repairs if required" };
            AddRow(row++, "Note", NoteField);
        }
    }

    // ─────────────────────────────────────────────
    // Lease Form
    // ─────────────────────────────────────────────
    public class LeaseFormTab : FormTab
    {
        private TextBox? DeviceField;
        private TextBox? DeviceColorField;
        private TextBox? DeviceStorageField;
        private TextBox? ImeiSerialField;
        private TextBox? DeviceConditionField;
        private TextBox? AccessoriesField;
        private TextBox? BorrowerNameField;
        private TextBox? BorrowerContactField;
        private TextBox? BorrowerAddressField;
        private TextBox? BorrowerIdField;
        private TextBox? StaffNameField;
        private DatePicker? DateField;

        protected override void BuildForm()
        {
            int row = 0;
            DeviceField = new TextBox { Watermark = "Device name and model" };
            AddRow(row++, "Device", DeviceField);

            DeviceColorField = new TextBox { Watermark = "Blue/Red/Purple..." };
            AddRow(row++, "Device Color", DeviceColorField);

            DeviceStorageField = new TextBox { Watermark = "512 GB" };
            AddRow(row++, "Device Storage", DeviceStorageField);

            ImeiSerialField = new TextBox { Watermark = "*#06#" };
            AddRow(row++, "IMEI/Serial", ImeiSerialField);

            DeviceConditionField = new TextBox { Watermark = "Mint/Telstra only..." };
            AddRow(row++, "Device Condition", DeviceConditionField);

            AccessoriesField = new TextBox { Watermark = "Screen protector/case..." };
            AddRow(row++, "Accessories", AccessoriesField);

            BorrowerNameField = new TextBox { Watermark = "Name" };
            AddRow(row++, "Borrower's Name", BorrowerNameField);

            BorrowerContactField = new TextBox { Watermark = "04XY XYZ XYZ" };
            AddRow(row++, "Borrower's Contact", BorrowerContactField);

            BorrowerAddressField = new TextBox { Watermark = "1/23 John Whiteway Dr, Nth Avoca, NSW 2257" };
            AddRow(row++, "Borrower's Address", BorrowerAddressField);

            BorrowerIdField = new TextBox { Watermark = "2323452" };
            AddRow(row++, "Borrower's ID", BorrowerIdField);

            StaffNameField = new TextBox { Watermark = "Your name" };
            AddRow(row++, "Staff Name", StaffNameField);

            DateField = new DatePicker()
            {
                HorizontalAlignment = HorizontalAlignment.Stretch,
            };
            AddRow(row++, "Date", DateField);
        }
    }

    // ─────────────────────────────────────────────
    // Risk Forms
    // ─────────────────────────────────────────────
    public abstract class RiskFormTab : FormTab
    {
        protected TextBox? CustomerNameField;
        protected TextBox? DeviceField;

        protected override void BuildForm()
        {
            int row = 0;
            CustomerNameField = new TextBox { Watermark = "Customer Name            " };
            AddRow(row++, "Customer Name", CustomerNameField);

            DeviceField = new TextBox { Watermark = "iPhone 14 Pro Max         " };
            AddRow(row++, "Device", DeviceField);
        }
    }

    public class FragileScreenFormTab : RiskFormTab { }
    public class BackGlassFormTab : RiskFormTab { }

    // ─────────────────────────────────────────────
    // Forms Scene
    // ─────────────────────────────────────────────
    public class FormsScene : BaseScene
    {
        public FormsScene(Window parentWindow) : base("Forms", parentWindow)
        {
            var tabControl = new TabControl
            {
                TabStripPlacement = Dock.Left,
                HorizontalAlignment = HorizontalAlignment.Center,
                VerticalAlignment = VerticalAlignment.Center
            };

            var tabs = new List<TabItem>
            {
                CreateTab("Refurbished Sale", new RefurbishedDeviceSaleTab()),
                CreateTab("New Device Sale", new NewDeviceSaleTab()),
                CreateTab("Purchase", new DevicePurchaseTab()),
                CreateTab("Lease", new LeaseFormTab()),
                CreateTab("Fragile Screen", new FragileScreenFormTab()),
                CreateTab("Back Glass", new BackGlassFormTab())
            };

            tabControl.ItemsSource = tabs;
            ContentPanel.Children.Add(tabControl);
        }

        private TabItem CreateTab(string header, Control content)
        {
            return new TabItem
            {
                Header = new TextBlock
                {
                    Text = header,
                    FontSize = 14,
                    HorizontalAlignment = HorizontalAlignment.Center
                },
                Content = new Border  // Add some padding around the content
                {
                    Padding = new Thickness(10),
                    Child = content
                }
            };
        }
    }
}