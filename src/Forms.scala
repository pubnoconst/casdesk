import scalafx.scene.Scene

import scalafx.scene.Scene
import scalafx.scene.control.{Button, Label, Tab, TabPane}
import scalafx.scene.layout.VBox
import scalafx.geometry.Pos

class Forms extends Scene:
  val backBtn = new Button("Back")
  root = new VBox {
    spacing = 20
    alignment = Pos.Center
    prefHeight = 600

    children = Seq(
      new TabPane {
        tabs = Seq(
          new Tab {
            text = "Sale"
            content = new Label("Sale Form")
            closable = false
          },
          new Tab {
            text = "Purchase"
            content = new Label("Purchase Form")
            closable = false
          },
          new Tab {
            text = "Loan"
            content = new Label("Loan Form")
            closable = false
          }
        )
      },
      backBtn
    )
  }