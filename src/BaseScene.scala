import scalafx.scene.Scene
import scalafx.scene.control.Button
import scalafx.scene.layout.Region
import scalafx.scene.layout.Priority
import scalafx.scene.layout.HBox
import scalafx.geometry.Pos
import scalafx.scene.control.Label
import scalafx.scene.Node
import scalafx.scene.layout.VBox
import scalafx.geometry.Insets
import atlantafx.base.theme.Styles

abstract class BaseScene(name: String) extends Scene:
    var homeBtn = Button("Back")
    homeBtn.getStyleClass().add(Styles.BUTTON_OUTLINED)
    private val spacer = new Region()
    HBox.setHgrow(spacer, Priority.Always)
    val navHbox = new HBox {
        spacing = 10
        alignment = Pos.Center
        children = Seq(homeBtn, spacer, new Label(name){
            style = "-fx-font-size: 20px;"
        })
    }
    var contentBox = VBox()
    contentBox.setAlignment(Pos.TopCenter)
    VBox.setVgrow(contentBox, Priority.Always)
    root = new VBox {
        spacing = 20
        alignment = Pos.TopCenter
        padding = Insets(25)
        children = Seq(navHbox, contentBox)
    }
