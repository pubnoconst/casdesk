import scalafx.scene.Scene
import scalafx.scene.control.{Button, Label, Tab, TabPane}
import scalafx.geometry.{Insets, Pos}
import scalafx.scene.layout.{VBox, HBox, GridPane}
import scalafx.scene.control.TextField
import scalafx.scene.control.DatePicker
import scalafx.scene.control.ScrollPane
import atlantafx.base.theme.Styles

import java.time.LocalDate
import java.time.format.DateTimeFormatter

// Returns the current date as a string in "dd/MM/yyyy" format.
def currentDateString(): String =
  val formatter = DateTimeFormatter.ofPattern("dd/MM/yyyy")
  LocalDate.now.format(formatter)

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
    alignment = Pos.Center
    children = formGrid
  }

  // Wrap the centered content in a ScrollPane
  content = new ScrollPane {
    content = centeredContent
    fitToWidth = true
    fitToHeight = true
  }
}

abstract class DeviceSaleTab(header: String)
    extends FormTab(header, new GridPane, new Button("Submit")):
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  protected val customerNameField    = new TextField()
  protected val deviceField          = new TextField()
  protected val deviceColorField     = new TextField()
  protected val deviceImeiField      = new TextField()
  protected val deviceProviderField  = new TextField()
  protected val devicePriceField     = new TextField()
  protected val paymentMethodField   = new TextField()
  protected val customerContactField = new TextField()
  protected val customerAddressField = new TextField()
  protected val customerIdField      = new TextField()
  protected val staffNameField       = new TextField()
  protected val dateField            = new DatePicker()

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

  // Default submit handler (can be overridden by subclasses)
  submitBtn.onAction = _ => {
    val dateStr = Option(dateField.value.value).map(_.toString).getOrElse(currentDateString())
    println(s"$headerText form submitted with data:")
    println(s"Customer Name: ${customerNameField.text.value}")
    // Additional processing can be done here.
  }

  formGrid.add(submitBtn, 1, 12)

class RefurbishedDeviceSaleTab extends DeviceSaleTab("Refurbished Device Sale"):
  // On submit, call the templating function on a virtual thread.
  submitBtn.onAction = _ =>
    val dateStr = Option(dateField.value.value).map(_.toString).getOrElse(currentDateString())
    Thread.startVirtualThread { () =>
      // Call the refurbishedDeviceSaleForm function from Templating.scala.
      refurbishedDeviceSaleForm(
        customerNameField.text.value,
        deviceField.text.value,
        deviceColorField.text.value,
        deviceProviderField.text.value,
        deviceImeiField.text.value,
        devicePriceField.text.value,
        paymentMethodField.text.value,
        customerContactField.text.value,
        customerAddressField.text.value,
        customerIdField.text.value,
        dateStr,
        staffNameField.text.value
      )
    }
    println("Refurbished Device Sale form submitted.")

class NewDeviceSaleTab extends DeviceSaleTab("New Device Sale"):
  submitBtn.onAction = _ =>
    val dateStr = Option(dateField.value.value).map(_.toString).getOrElse(currentDateString())
    Thread.startVirtualThread { () =>
      newDeviceSaleForm(
        customerNameField.text.value,
        deviceField.text.value,
        deviceColorField.text.value,
        deviceProviderField.text.value,
        deviceImeiField.text.value,
        devicePriceField.text.value,
        paymentMethodField.text.value,
        customerContactField.text.value,
        customerAddressField.text.value,
        customerIdField.text.value,
        dateStr,
        staffNameField.text.value
      )
    }
    println("New Device Sale form submitted.")

class DevicePurchaseTab
    extends FormTab(
      headerText = "Device Purchase",
      formGrid = new GridPane,
      submitBtn = new Button("Submit")
    ) {
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  protected val sellerNameField    = new TextField()
  protected val deviceField        = new TextField()
  protected val deviceColorField   = new TextField()
  protected val memoryField        = new TextField()
  protected val imeiField          = new TextField()
  protected val deviceProviderField= new TextField()
  protected val purchasePriceField = new TextField()
  protected val sellerContactField = new TextField()
  protected val sellerAddressField = new TextField()
  protected val sellerIdField      = new TextField()
  protected val staffNameField     = new TextField()
  protected val dateField          = new DatePicker()
  protected val noteField          = new TextField()

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

  formGrid.add(submitBtn, 1, 13)

  submitBtn.onAction = _ =>
    val dateStr = Option(dateField.value.value).map(_.toString).getOrElse(currentDateString())
    Thread.startVirtualThread { () =>
      purchaseForm(
        sellerNameField.text.value,
        deviceField.text.value,
        purchasePriceField.text.value,
        memoryField.text.value,
        deviceColorField.text.value,
        deviceProviderField.text.value,
        imeiField.text.value,
        sellerContactField.text.value,
        sellerAddressField.text.value,
        sellerIdField.text.value,
        dateStr,
        staffNameField.text.value,
        noteField.text.value
      )
    }
    println("Device Purchase form submitted.")
}

class LeaseFormTab
    extends FormTab(
      headerText = "Lease Form",
      formGrid = new GridPane,
      submitBtn = new Button("Submit")
    ) {
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  protected val deviceField         = new TextField()
  protected val deviceColorField    = new TextField()
  protected val deviceStorageField  = new TextField()
  protected val imeiSerialField     = new TextField()
  protected val deviceConditionField= new TextField()
  protected val accessoriesField    = new TextField()
  protected val borrowerNameField   = new TextField()
  protected val borrowerContactField= new TextField()
  protected val borrowerAddressField= new TextField()
  protected val borrowerIdField     = new TextField()
  protected val staffNameField      = new TextField()
  protected val dateField           = new DatePicker()

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

  formGrid.add(new Label("Borrower's Address"), 0, 8)
  formGrid.add(borrowerAddressField, 1, 8)

  formGrid.add(new Label("Borrower's ID Number"), 0, 9)
  formGrid.add(borrowerIdField, 1, 9)

  formGrid.add(new Label("Staff Name"), 0, 10)
  formGrid.add(staffNameField, 1, 10)

  formGrid.add(new Label("Date"), 0, 11)
  formGrid.add(dateField, 1, 11)

  formGrid.add(submitBtn, 1, 12)
  submitBtn.onAction = _ => {
    val dateStr = Option(dateField.value.value).map(_.toString).getOrElse(currentDateString())
    Thread.startVirtualThread { () =>
      leaseDeviceForm(
        borrowerName = borrowerNameField.text.value,
        deviceName = deviceField.text.value,
        deviceStorage = deviceStorageField.text.value,
        deviceColor = deviceColorField.text.value,
        deviceImei = imeiSerialField.text.value,
        deviceCondition = deviceConditionField.text.value,
        accessories = accessoriesField.text.value,
        borrowerAddress = borrowerAddressField.text.value,
        borrowerContact = borrowerContactField.text.value,
        borrowerId = borrowerIdField.text.value,
        date = dateStr,
        staff = staffNameField.text.value
      )
    }
    println("Lease Form submitted.")
  }
}


abstract class RiskFormTab(
    headerText: String
) extends FormTab(
      headerText = headerText,
      formGrid = new GridPane,
      submitBtn = new Button("Submit")
    ) {
  formGrid.padding = Insets(10)
  formGrid.hgap = 10
  formGrid.vgap = 10

  // Only common fields for risk forms
  protected val customerNameField = new TextField()
  protected val deviceField       = new TextField()

  formGrid.add(new Label("Customer Name"), 0, 0)
  formGrid.add(customerNameField, 1, 0)

  formGrid.add(new Label("Device"), 0, 1)
  formGrid.add(deviceField, 1, 1)

  formGrid.add(submitBtn, 1, 2)
}

class FragileScreenFormTab extends RiskFormTab("Fragile Screen Form"):
  // No date input; use currentDateString() automatically.
  submitBtn.onAction = _ =>
    Thread.startVirtualThread { () =>
      fragileScreen(
        deviceField.text.value,
        customerNameField.text.value,
        currentDateString()
      )
    }
    println("Fragile Screen Form submitted.")

class BackGlassFormTab extends RiskFormTab("Back Glass Form"):
  // No date input; use currentDateString() automatically.
  submitBtn.onAction = _ =>
    Thread.startVirtualThread { () =>
      backGlass(
        deviceField.text.value,
        customerNameField.text.value,
        currentDateString()
      )
    }
    println("Back Glass Form submitted.")

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
