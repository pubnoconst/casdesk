import scala.sys.process.*
import java.awt.*
import java.awt.image.BufferedImage
import java.util.concurrent.Executors

def notifyOS(message: String): Unit = 
  val executor = Executors.newVirtualThreadPerTaskExecutor()
  executor.submit(() => 
    val osName = System.getProperty("os.name").toLowerCase

    if osName.contains("win") then
      if SystemTray.isSupported then
        val image = new BufferedImage(1, 1, BufferedImage.TYPE_INT_ARGB)
        val trayIcon = new TrayIcon(image, "Casdesk")
        trayIcon.setImageAutoSize(true)

        try 
          val tray = SystemTray.getSystemTray
          tray.add(trayIcon)
          trayIcon.displayMessage("", message, TrayIcon.MessageType.NONE)
          Thread.sleep(5000)
          tray.remove(trayIcon)
        catch case e: Exception => println("Error displaying Windows notification: " + e.getMessage)
      else println("System tray not supported!")

    else if osName.contains("mac") then
      val safeMessage = message.replace("\"", "\\\"")
      try Seq("osascript", "-e", s"""display notification "$safeMessage" with title "Casdesk"""").!
      catch case e: Exception => println("Error displaying macOS notification: " + e.getMessage)

    else if osName.contains("linux") then
      try Seq("notify-send", "Casdesk", message).!
      catch case e: Exception => println("Error displaying Linux notification: " + e.getMessage)

    else println("Unsupported OS: " + osName)
  )

