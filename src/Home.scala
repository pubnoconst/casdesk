import scalafx.scene.Scene
import scalafx.scene.layout.VBox
import scalafx.geometry.Pos
import scalafx.scene.control.Label
import scalafx.scene.layout.Priority
import scalafx.scene.control.Button
import scalafx.scene.layout.HBox

class Home extends Scene:
    val formsBtn = new Button("Forms")
    root = new VBox {
        spacing = 20
        alignment = Pos.Center
        children = Seq(
            new Label("Casdesk") {
                style = "-fx-font-size: 48pt;"
            },
            new HBox {
                spacing = 10
                alignment = Pos.Center
                children = Seq(
                    formsBtn,
                    new Button("Quotes") {
                    },
                    new Button("Adjust") {
                    }
                )
            }
        )
    }