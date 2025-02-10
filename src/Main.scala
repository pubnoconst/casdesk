//> using dep "org.scalafx::scalafx:23.0.1-R34"

import scalafx.application.JFXApp3
import scalafx.geometry.Insets
import scalafx.geometry.Pos
import scalafx.scene.Scene
import scalafx.scene.control._
import scalafx.scene.layout._
import scalafx.collections.ObservableBuffer
import scalafx.event.ActionEvent
import scalafx.Includes._

object QuoteFormsApp extends JFXApp3 {
  // Quote Scene
  private lazy val quoteScene: Scene = {
    val backButton = new Button("Back to Main") { // Back button for quote scene
      onAction = _ => stage.scene = mainScene
    }
    val partCosts = ObservableBuffer[Double]()

    // Card 1: Known Parts
    val deviceModelInput = new TextField {
      promptText = "Device/Model Number (Under Construction)"
      disable = true
    }
    val knownPartsCard = new VBox {
      children = Seq(
        new Label("Known Parts"),
        deviceModelInput,
        new Button("Confirm") { disable = true }
      )
      padding = Insets(20)
      spacing = 10
      style = "-fx-border-color: black; -fx-padding: 10;"
    }

    // Card 2: Calculate Quote
    val partCostInput = new TextField {
      promptText = "Part Cost"
    }

    val calculateQuoteCard = new VBox {
      children = Seq(
        new Label("Calculate Quote"),
        partCostInput,
        new Button("Confirm") {
          onAction = (ae: ActionEvent) =>
            try {
              val cost =
                partCostInput.text.value.toDouble // .value is crucial in ScalaFX
              partCosts.add(cost)
              partCostInput.clear()
              println(s"Added cost: $cost, Total costs: ${partCosts.sum}")
            } catch {
              case e: NumberFormatException =>
                println("Invalid part cost. Please enter a number.")
            }
        }
      )
      padding = Insets(20)
      spacing = 10
      style = "-fx-border-color: black; -fx-padding: 10;"
    }

    val costsListLabel = new Label("Part Costs:")
    val costsListView = new ListView[Double](partCosts)

    val quoteLayout = new VBox {
      children = Seq(
        backButton,
        knownPartsCard,
        calculateQuoteCard,
        new Label("Part Costs:"),
        new ListView[Double](partCosts)
      ) // Back button added
      alignment = Pos.Center
      padding = Insets(20)
      spacing = 20
    }
    new Scene(quoteLayout, 650, 650)
  }

  // Main Scene
  private lazy val mainScene: Scene = { // Make mainScene a lazy val as well
    // val cssStream = getClass.getResourceAsStream("primer-light.css")
    // require(cssStream != null, "Resource 'primer-light.css' not found!")
    // val cssContent = new String(cssStream.readAllBytes(), "UTF-8")
    // JFXApp3.userAgentStylesheet = "data:text/css," + cssContent

    val quoteButton = new Button("Quote") {
      onAction = _ => stage.scene = quoteScene
    }
    val formsButton = new Button("Forms") {
      // onAction = _ => stage.scene = formsScene // Switch to forms scene
    }
    val titleLabel = new Label("Casdesk")
    titleLabel.style = "-fx-font-size: 48pt;"

    val mainLayout = new VBox {
      children = Seq(
        titleLabel,
        new HBox {
          children = Seq(quoteButton, formsButton)
          alignment = Pos.Center
          spacing = 20
        }
      )
      alignment = Pos.Center
      padding = Insets(20)
      spacing = 20
    }
    new Scene(mainLayout, 650, 650)
  }

  override def start(): Unit =
    stage = new JFXApp3.PrimaryStage {
      title = "Casdesk"
      scene = mainScene // Set initial scene to mainScene
      minHeight = 650
      minWidth = 650
    }
}
