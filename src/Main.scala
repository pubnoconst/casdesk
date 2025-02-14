//> using dep org.scalafx::scalafx:23.0.1-R34
//> using dep io.github.mkpaz:atlantafx-base:2.0.1

//> using jvm 21
//> using scala 3.6.2

import scalafx.application.JFXApp3
import scalafx.geometry.Insets
import scalafx.geometry.Pos
import scalafx.scene.Scene
import scalafx.scene.control._
import scalafx.scene.layout._
import scalafx.collections.ObservableBuffer
import scalafx.event.ActionEvent
import scalafx.Includes._
import javafx.application.Application
import atlantafx.base.theme.PrimerLight

object MainApp extends JFXApp3:
  override def start(): Unit = 
    Application.setUserAgentStylesheet(PrimerLight().getUserAgentStylesheet())
    val formsScene = new Forms()
    val homeScene = new Home()
    stage = new JFXApp3.PrimaryStage:
      title = "Casdesk"
      scene = homeScene
      minWidth = 600
      minHeight = 600

    homeScene.formsBtn.onAction = (e: ActionEvent) => stage.scene = formsScene
    formsScene.backBtn.onAction = (e: ActionEvent) => stage.scene = homeScene
