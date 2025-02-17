import scalafx.scene.Scene
import scalafx.scene.control.{Button, Label, Tab, TabPane}
import scalafx.geometry.{Insets, Pos, Side}
import scalafx.scene.layout.{VBox, HBox, Priority}

class Forms extends Scene:
  val backBtn = new Button("Back")

  // First HBox (should not grow)
  val firstHBox = new HBox {
    spacing = 10
    alignment = Pos.Center
    children = Seq(backBtn, new Label("Forms"))
  }

  // Styled function to create better tab headers
  def createTab(labelText: String, contentText: String): Tab = new Tab {
    graphic = new HBox {
      alignment = Pos.Center
      children = Seq(new Label(labelText))
    }
    content = new Label(contentText)
    closable = false
  }

  // Second HBox (should take remaining space)
  val secondHBox = new HBox {
    children = Seq(new TabPane {
      side = Side.LEFT
      tabMinHeight = 100
      tabs = Seq(
        createTab("Sale", "Sale Form"),
        createTab("Purchase", "Purchase Form"),
        createTab("Loan", "Loan Form")
      )
    })
  }
  VBox.setVgrow(secondHBox, Priority.ALWAYS) // Make second HBox expand

  // Root VBox (should take full scene)
  root = new VBox {
    spacing = 20
    alignment = Pos.Center
    padding = Insets(25)
    children = Seq(firstHBox, secondHBox)
  }
