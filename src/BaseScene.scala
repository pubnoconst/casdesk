import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.layout.Region
import scalafx.scene.layout.Priority
import scalafx.scene.layout.HBox
import scalafx.geometry.Pos
import scalafx.scene.control.Label

abstract class BaseScene(name: String) extends Scene:
    var homeBtn = Button("Back")
    private val spacer = new Region()
    HBox.setHgrow(spacer, Priority.ALWAYS)
    val navHbox = new HBox {
        spacing = 10
        alignment = Pos.Center
        children = Seq(homeBtn, spacer, new Label(name){
            style = "-fx-font-size: 20px;"
        })
    }
