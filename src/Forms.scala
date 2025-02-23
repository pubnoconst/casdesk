import scalafx.scene.Scene
import scalafx.scene.control.{Button, Label, Tab, TabPane}
import scalafx.geometry.{Insets, Pos, Side}
import scalafx.scene.layout.{VBox, HBox, Priority}
import scalafx.scene.text.Text
import scalafx.scene.layout.Region
import scalafx.scene.layout.GridPane
import scalafx.scene.control.TextField
import scalafx.scene.control.TextFormatter
import scalafx.util.converter.BigDecimalStringConverter
import scalafx.scene.control.DatePicker

import scalafx.scene.control.ScrollPane

import scalafx.scene.layout.HBox
import atlantafx.base.theme.Styles

abstract class FormTab(
    val headerText: String,
    val formGrid: GridPane,
    val submitBtn: Button = new Button("Confirm")
) extends Tab {
  closable = false
  text = headerText // Set the tab header text

  submitBtn.getStyleClass().add(Styles.ACCENT)

  // Wrap the formGrid in an HBox to center it horizontally
  protected val centeredContent = new HBox {
    alignment = Pos.Center // Center the content horizontally
    children = formGrid
  }

  // Wrap the centered content in a ScrollPane
  content = new ScrollPane {
    content = centeredContent
    fitToWidth = true // Ensures the content fits the width of the ScrollPane
    fitToHeight = true // Ensures the content fits the height of the ScrollPane
  }
}

abstract class DeviceSaleTab(header: String)
    extends FormTab(header, new GridPane, new Button("Submit")):
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  protected val customerNameField = new TextField()
  protected val deviceField = new TextField()
  protected val deviceColorField = new TextField()
  protected val deviceImeiField = new TextField()
  protected val deviceProviderField = new TextField()
  protected val devicePriceField = new TextField()
  protected val paymentMethodField = new TextField()
  protected val customerContactField = new TextField()
  protected val customerAddressField = new TextField()
  protected val customerIdField = new TextField()
  protected val staffNameField = new TextField()
  protected val dateField = new DatePicker()

  formGrid.add(new Label("Customer Name"), 0, 0)
  formGrid.add(customerNameField, 1, 0)

  formGrid.add(new Label("Device"), 0, 1)
  formGrid.add(deviceField, 1, 1)

  formGrid.add(new Label("Device Color"), 0, 2)
  formGrid.add(deviceColorField, 1, 2)

  formGrid.add(new Label("Device IMEI"), 0, 3)
  formGrid.add(deviceImeiField, 1, 3)

  formGrid.add(new Label("Device Provider"), 0, 4)
  formGrid.add(deviceProviderField, 1, 4)

  formGrid.add(new Label("Device Price"), 0, 5)
  formGrid.add(devicePriceField, 1, 5)

  formGrid.add(new Label("Payment Method"), 0, 6)
  formGrid.add(paymentMethodField, 1, 6)

  formGrid.add(new Label("Customer Contact Number"), 0, 7)
  formGrid.add(customerContactField, 1, 7)

  formGrid.add(new Label("Customer Address"), 0, 8)
  formGrid.add(customerAddressField, 1, 8)

  formGrid.add(new Label("Customer ID Number"), 0, 9)
  formGrid.add(customerIdField, 1, 9)

  formGrid.add(new Label("Staff Name"), 0, 10)
  formGrid.add(staffNameField, 1, 10)

  formGrid.add(new Label("Date"), 0, 11)
  formGrid.add(dateField, 1, 11)

  submitBtn.onAction = _ => {
    val formData = Map(
      "Customer Name" -> customerNameField.text.value,
      "Device" -> deviceField.text.value,
      "Device Color" -> deviceColorField.text.value,
      "Device IMEI" -> deviceImeiField.text.value,
      "Device Provider" -> deviceProviderField.text.value,
      "Device Price" -> devicePriceField.text.value,
      "Payment Method" -> paymentMethodField.text.value,
      "Customer Contact Number" -> customerContactField.text.value,
      "Customer Address" -> customerAddressField.text.value,
      "Customer ID Number" -> customerIdField.text.value,
      "Staff Name" -> staffNameField.text.value,
      "Date" -> {
        try dateField.value.value.toString
        catch case _ => {
          notifyOS("Invalid date")
          ""
        }
      }
    )
    println(s"$header form submitted with data:")
    formData.foreach { case (key, value) => println(s"$key: $value") }
  }

  formGrid.add(submitBtn, 1, 12)

class RefurbishedDeviceSaleTab extends DeviceSaleTab("Refurbished Device Sale")

class NewDeviceSaleTab extends DeviceSaleTab("New Device Sale")

class DevicePurchaseTab
    extends FormTab(
      headerText = "Device Purchase", // Tab header text
      formGrid = new GridPane, // GridPane for the form layout
      submitBtn = new Button("Submit") // Submit button
    ) {
  // Set padding and gaps for the form grid
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  // Define form fields
  protected val sellerNameField = new TextField()
  protected val deviceField = new TextField()
  protected val deviceColorField = new TextField()
  protected val memoryField = new TextField()
  protected val imeiField = new TextField()
  protected val deviceProviderField = new TextField()
  protected val purchasePriceField = new TextField()
  protected val sellerContactField = new TextField()
  protected val sellerAddressField = new TextField()
  protected val sellerIdField = new TextField()
  protected val staffNameField = new TextField()
  protected val dateField = new DatePicker()
  protected val noteField = new TextField()

  // Add fields to the form grid
  formGrid.add(new Label("Seller's Name"), 0, 0)
  formGrid.add(sellerNameField, 1, 0)

  formGrid.add(new Label("Device"), 0, 1)
  formGrid.add(deviceField, 1, 1)

  formGrid.add(new Label("Device Color"), 0, 2)
  formGrid.add(deviceColorField, 1, 2)

  formGrid.add(new Label("Memory"), 0, 3)
  formGrid.add(memoryField, 1, 3)

  formGrid.add(new Label("IMEI"), 0, 4)
  formGrid.add(imeiField, 1, 4)

  formGrid.add(new Label("Device Provider"), 0, 5)
  formGrid.add(deviceProviderField, 1, 5)

  formGrid.add(new Label("Purchase Price"), 0, 6)
  formGrid.add(purchasePriceField, 1, 6)

  formGrid.add(new Label("Seller's Contact Number"), 0, 7)
  formGrid.add(sellerContactField, 1, 7)

  formGrid.add(new Label("Seller's Address"), 0, 8)
  formGrid.add(sellerAddressField, 1, 8)

  formGrid.add(new Label("Seller's ID Number"), 0, 9)
  formGrid.add(sellerIdField, 1, 9)

  formGrid.add(new Label("Staff Name"), 0, 10)
  formGrid.add(staffNameField, 1, 10)

  formGrid.add(new Label("Date"), 0, 11)
  formGrid.add(dateField, 1, 11)

  formGrid.add(new Label("Note for Office"), 0, 12)
  formGrid.add(noteField, 1, 12)

  // Add the submit button to the form grid
  formGrid.add(submitBtn, 1, 13)

  // Define the action for the submit button
  submitBtn.onAction = _ => {
    val formData = Map(
      "Seller's Name" -> sellerNameField.text.value,
      "Device" -> deviceField.text.value,
      "Device Color" -> deviceColorField.text.value,
      "Memory" -> memoryField.text.value,
      "IMEI" -> imeiField.text.value,
      "Device Provider" -> deviceProviderField.text.value,
      "Purchase Price" -> purchasePriceField.text.value,
      "Seller's Contact Number" -> sellerContactField.text.value,
      "Seller's Address" -> sellerAddressField.text.value,
      "Seller's ID Number" -> sellerIdField.text.value,
      "Staff Name" -> staffNameField.text.value,
      "Date" -> {
        try dateField.value.value.toString
        catch case _ => {
          notifyOS("Invalid OS")
          ""
        }
      },
      "Note for Office" -> noteField.text.value
    )

    // Print the form data to the console
    println(s"Device Purchase form submitted with data:")
    formData.foreach { case (key, value) => println(s"$key: $value") }
  }
}

class LeaseFormTab
    extends FormTab(
      headerText = "Lease Form", // Tab header text
      formGrid = new GridPane, // GridPane for the form layout
      submitBtn = new Button("Submit") // Submit button
    ) {
  // Set padding and gaps for the form grid
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  // Define form fields
  protected val deviceField = new TextField()
  protected val deviceColorField = new TextField()
  protected val deviceStorageField = new TextField()
  protected val imeiSerialField = new TextField()
  protected val deviceConditionField = new TextField()
  protected val accessoriesField = new TextField()
  protected val borrowerNameField = new TextField()
  protected val borrowerContactField = new TextField()
  protected val borrowerIdField = new TextField()
  protected val staffNameField = new TextField()
  protected val dateField = new DatePicker()

  // Add fields to the form grid
  formGrid.add(new Label("Device"), 0, 0)
  formGrid.add(deviceField, 1, 0)

  formGrid.add(new Label("Device Color"), 0, 1)
  formGrid.add(deviceColorField, 1, 1)

  formGrid.add(new Label("Device Storage"), 0, 2)
  formGrid.add(deviceStorageField, 1, 2)

  formGrid.add(new Label("IMEI/Serial Number"), 0, 3)
  formGrid.add(imeiSerialField, 1, 3)

  formGrid.add(new Label("Device Condition"), 0, 4)
  formGrid.add(deviceConditionField, 1, 4)

  formGrid.add(new Label("Accessories"), 0, 5)
  formGrid.add(accessoriesField, 1, 5)

  formGrid.add(new Label("Borrower's Name"), 0, 6)
  formGrid.add(borrowerNameField, 1, 6)

  formGrid.add(new Label("Borrower's Contact Number"), 0, 7)
  formGrid.add(borrowerContactField, 1, 7)

  formGrid.add(new Label("Borrower's ID Number"), 0, 8)
  formGrid.add(borrowerIdField, 1, 8)

  formGrid.add(new Label("Staff Name"), 0, 9)
  formGrid.add(staffNameField, 1, 9)

  formGrid.add(new Label("Date"), 0, 10)
  formGrid.add(dateField, 1, 10)

  // Add the submit button to the form grid
  formGrid.add(submitBtn, 1, 11)

  // Define the action for the submit button
  submitBtn.onAction = _ => {
    val formData = Map(
      "Device" -> deviceField.text.value,
      "Device Color" -> deviceColorField.text.value,
      "Device Storage" -> deviceStorageField.text.value,
      "IMEI/Serial Number" -> imeiSerialField.text.value,
      "Device Condition" -> deviceConditionField.text.value,
      "Accessories" -> accessoriesField.text.value,
      "Borrower's Name" -> borrowerNameField.text.value,
      "Borrower's Contact Number" -> borrowerContactField.text.value,
      "Borrower's ID Number" -> borrowerIdField.text.value,
      "Staff Name" -> staffNameField.text.value,
      "Date" -> {
        try dateField.value.value.toString
        catch case _ =>  {
          notifyOS("Invalid OS")
          ""
        }
      }
    )

    // Print the form data to the console
    println(s"Lease Form submitted with data:")
    formData.foreach { case (key, value) => println(s"$key: $value") }
  }
}

abstract class RiskFormTab(
    headerText: String
) extends FormTab(
      headerText = headerText,
      formGrid = new GridPane,
      submitBtn = new Button("Submit")
    ) {
  // Set padding and gaps for the form grid
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  // Common fields for all risk forms
  protected val customerNameField = new TextField()
  protected val deviceField = new TextField()

  // Add common fields to the form grid
  formGrid.add(new Label("Customer Name"), 0, 0)
  formGrid.add(customerNameField, 1, 0)

  formGrid.add(new Label("Device"), 0, 1)
  formGrid.add(deviceField, 1, 1)

  // Add the submit button to the form grid
  formGrid.add(submitBtn, 1, 2)

  // Define the action for the submit button
  submitBtn.onAction = _ => {
    val formData = Map(
      "Customer Name" -> customerNameField.text.value,
      "Device" -> deviceField.text.value
    )

    // Print the form data to the console
    println(s"$headerText form submitted with data:")
    formData.foreach { case (key, value) => println(s"$key: $value") }
  }
}

class FragileScreenFormTab
    extends RiskFormTab(
      headerText = "Fragile Screen Form"
    )

class BackGlassFormTab
    extends RiskFormTab(
      headerText = "Back Glass Form"
    )

class Forms extends BaseScene("Forms"):
  val tabBox = new VBox {
    children = Seq(new TabPane {
      tabs = Seq(
        RefurbishedDeviceSaleTab(),
        NewDeviceSaleTab(),
        DevicePurchaseTab(),
        LeaseFormTab(),
        FragileScreenFormTab(),
        BackGlassFormTab()
      )
    })
  }

  tabBox.setAlignment(Pos.Center)
  contentBox.children = Seq(tabBox)
