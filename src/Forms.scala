import scalafx.scene.Scene
import scalafx.scene.control.{Button, Label, Tab, TabPane}
import scalafx.geometry.{Insets, Pos, Side}
import scalafx.scene.layout.{VBox, HBox, Priority}
import scalafx.scene.text.Text
import scalafx.scene.layout.Region

class Forms extends BaseScene("Forms"):
  // Styled function to create better tab headers
  def createTab(labelText: String, contentText: String): Tab = new Tab {
    graphic = new Text(labelText)
    content = new Label(contentText)
    closable = false
  }

  // Second HBox (should take remaining space)
  val tabBox = new HBox {
    children = Seq(new TabPane {
      tabs = Seq(
        createTab("Refurbsihsed device sale form", "Sale Pre-owned device Form"),
        createTab("New device sale form", "Sale New Device Form"),
        createTab("Purchase of a device", "Purchase Form"),
        createTab("Lease device", "Device lease Form"),
        createTab("Fragile glass form", "Fragile glass"),
        createTab("Back glass form", "Back glass"),
      )
    })
  }
  tabBox.setAlignment(Pos.Center)
  contentBox.children = Seq(tabBox)