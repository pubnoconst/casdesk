import scala.io.Source
import java.nio.charset.StandardCharsets
import java.nio.file.{Files, Paths}
import java.util.Base64

// Cache the Base64-encoded logo from resources/logobanner.png
lazy val logoBannerBase64: String = {
  try {
    val classLoader = Thread.currentThread().getContextClassLoader
    val resourceUrl = classLoader.getResource("logobanner.png")
    require(resourceUrl != null, "Resource logobanner.png not found")
    val path = Paths.get(resourceUrl.toURI)
    val bytes = Files.readAllBytes(path)
    Base64.getEncoder.encodeToString(bytes)
  } catch {
    case e: Exception =>
      notifyOS(s"Error loading logobanner.png: ${e.getMessage}")
      ""
  }
}

// Reads the HTML file from the given resource path, injects the logo if not provided,
// applies the given replacements, and returns the resulting content.
def replaceHtmlPlaceholders(resourcePath: String, replacements: Array[(String, String)]): String = {
  try {
    val classLoader = Thread.currentThread().getContextClassLoader
    val resourceUrl = classLoader.getResource(resourcePath)
    require(resourceUrl != null, s"Resource not found: $resourcePath")

    val source = Source.fromURL(resourceUrl, StandardCharsets.UTF_8.name())
    val content = try source.mkString finally source.close()

    // Ensure the logo is injected if not explicitly provided
    val fullReplacements =
      if (replacements.exists(_._1 == "%LOGO_BANNER%")) replacements
      else replacements :+ ("%LOGO_BANNER%" -> logoBannerBase64)

    fullReplacements.foldLeft(content) { case (updatedContent, (key, value)) =>
      updatedContent.replace(key, value)
    }
  } catch {
    case e: Exception =>
      notifyOS(s"Error processing file $resourcePath: ${e.getMessage}")
      ""
  }
}

// Open an HTML file in the appropriate browser based on the OS.
def openHtmlFile(file: java.io.File): Unit = {
  try {
    val osName = System.getProperty("os.name").toLowerCase
    val filePath = file.getAbsolutePath
    val cmd: Array[String] =
      if (osName.contains("win"))
        // Assuming 'msedge' is available on Windows
        Array("cmd", "/c", "start", "msedge", filePath)
      else if (osName.contains("mac"))
        Array("open", "-a", "Safari", filePath)
      else
        // Assume Linux
        Array("xdg-open", filePath)
    new ProcessBuilder(cmd*).start()
  } catch {
    case e: Exception =>
      notifyOS(s"Error opening file ${file.getAbsolutePath}: ${e.getMessage}")
  }
}

// Helper that generates the HTML from a template, writes it to a temporary file, and opens it.
def generateAndOpenHtml(resourcePath: String, replacements: Array[(String, String)], filePrefix: String): Unit = {
  try {
    val content = replaceHtmlPlaceholders(resourcePath, replacements)
    if (content.nonEmpty) {
      val tempFile = Files.createTempFile(filePrefix, ".html").toFile
      Files.write(tempFile.toPath, content.getBytes(StandardCharsets.UTF_8))
      openHtmlFile(tempFile)
    }
  } catch {
    case e: Exception =>
      notifyOS(s"Error generating HTML for $resourcePath: ${e.getMessage}")
  }
}

// Each function spawns a virtual thread to process its template separately.

// back_glass_form.html
def backGlass(deviceModel: String, customerName: String, date: String): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/back_glass_form.html"
    val replacements = Array(
      "%DEVICE_MODEL%"  -> deviceModel,
      "%CUSTOMER_NAME%" -> customerName,
      "%DATE%"          -> date
    )
    generateAndOpenHtml(htmlFile, replacements, "backGlass_")
  })
}

// fragile_screen_form.html
def fragileScreen(deviceModel: String, customerName: String, date: String): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/fragile_screen_form.html"
    val replacements = Array(
      "%DEVICE_MODEL%"  -> deviceModel,
      "%CUSTOMER_NAME%" -> customerName,
      "%DATE%"          -> date
    )
    generateAndOpenHtml(htmlFile, replacements, "fragileScreen_")
  })
}

// lease_device_form.html
def leaseDeviceForm(
  borrowerName: String,
  deviceName: String,
  deviceStorage: String,
  deviceColor: String,
  deviceImei: String,
  deviceCondition: String,
  accessories: String,
  borrowerAddress: String,
  borrowerContact: String,
  borrowerId: String,
  date: String,
  staff: String
): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/lease_device_form.html"
    val replacements = Array(
      "%BORROWER_NAME%"    -> borrowerName,
      "%DEVICE_NAME%"      -> deviceName,
      "%DEVICE_STORAGE%"   -> deviceStorage,
      "%DEVICE_COLOR%"     -> deviceColor,
      "%DEVICE_IMEI%"      -> deviceImei,
      "%DEVICE_CONDITION%" -> deviceCondition,
      "%ACCESSORIES%"      -> accessories,
      "%BORROWER_ADDRESS%" -> borrowerAddress,
      "%BORROWER_CONTACT%" -> borrowerContact,
      "%BORROWER_ID%"      -> borrowerId,
      "%DATE%"             -> date,
      "%STAFF%"            -> staff
    )
    generateAndOpenHtml(htmlFile, replacements, "leaseDeviceForm_")
  })
}

// new_device_sale_form.html
def newDeviceSaleForm(
  customerName: String,
  deviceName: String,
  deviceColor: String,
  deviceLocked: String,
  deviceImei: String,
  devicePrice: String,
  paymentMethod: String,
  customerContact: String,
  customerAddress: String,
  customerId: String,
  date: String,
  staff: String
): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/new_device_sale_form.html"
    val replacements = Array(
      "%DEVICE_NAME%"      -> deviceName,
      "%DEVICE_COLOR%"     -> deviceColor,
      "%DEVICE_LOCKED%"    -> deviceLocked,
      "%DEVICE_IMEI%"      -> deviceImei,
      "%DEVICE_PRICE%"     -> devicePrice,
      "%PAYMENT_METHOD%"   -> paymentMethod,
      "%CUSTOMER_NAME%"    -> customerName,
      "%CUSTOMER_CONTACT%" -> customerContact,
      "%CUSTOMER_ADDRESS%" -> customerAddress,
      "%CUSTOMER_ID%"      -> customerId,
      "%DATE%"             -> date,
      "%STAFF%"            -> staff
    )
    generateAndOpenHtml(htmlFile, replacements, "newDeviceSaleForm_")
  })
}

// purchase_form.html
def purchaseForm(
  sellerName: String,
  deviceName: String,
  price: String,
  deviceMemory: String,
  deviceColor: String,
  deviceLocked: String,
  deviceImei: String,
  sellerContact: String,
  sellerAddress: String,
  customerId: String,
  date: String,
  staff: String,
  notes: String
): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/purchase_form.html"
    val replacements = Array(
      "%SELLER_NAME%"    -> sellerName,
      "%DEVICE_NAME%"    -> deviceName,
      "%PRICE%"          -> price,
      "%DEVICE_MEMORY%"  -> deviceMemory,
      "%DEVICE_COLOR%"   -> deviceColor,
      "%DEVICE_LOCKED%"  -> deviceLocked,
      "%DEVICE_IMEI%"    -> deviceImei,
      "%SELLER_CONTACT%" -> sellerContact,
      "%SELLER_ADDRESS%" -> sellerAddress,
      "%CUSTOMER_ID%"    -> customerId,
      "%DATE%"           -> date,
      "%STAFF%"          -> staff,
      "%NOTES%"          -> notes
    )
    generateAndOpenHtml(htmlFile, replacements, "purchaseForm_")
  })
}

// refurbished_device_sale_form.html
def refurbishedDeviceSaleForm(
  customerName: String,
  deviceName: String,
  deviceColor: String,
  deviceLocked: String,
  deviceImei: String,
  devicePrice: String,
  paymentMethod: String,
  customerContact: String,
  customerAddress: String,
  customerId: String,
  date: String,
  staff: String
): Unit = {
  Thread.startVirtualThread(() => {
    val htmlFile = "mockups/refurbished_device_sale_form.html"
    val replacements = Array(
      "%DEVICE_NAME%"      -> deviceName,
      "%DEVICE_COLOR%"     -> deviceColor,
      "%DEVICE_LOCKED%"    -> deviceLocked,
      "%DEVICE_IMEI%"      -> deviceImei,
      "%DEVICE_PRICE%"     -> devicePrice,
      "%PAYMENT_METHOD%"   -> paymentMethod,
      "%CUSTOMER_NAME%"    -> customerName,
      "%CUSTOMER_CONTACT%" -> customerContact,
      "%CUSTOMER_ADDRESS%" -> customerAddress,
      "%CUSTOMER_ID%"      -> customerId,
      "%DATE%"             -> date,
      "%STAFF%"            -> staff
    )
    generateAndOpenHtml(htmlFile, replacements, "refurbishedDeviceSaleForm_")
  })
}
