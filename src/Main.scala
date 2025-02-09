//> using dep "org.scalafx::scalafx:23.0.1-R34" 
//> using resourceDir "../assets"

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
    val partCosts = ObservableBuffer[Double]()

    // Card 1: Known Parts
    val deviceModelInput = new TextField {
      promptText = "Device/Model Number (Under Construction)"
      disable = true
    }
    val knownPartsCard = new VBox {
      children = Seq(new Label("Known Parts"), deviceModelInput, new Button("Confirm") { disable = true })
      padding = Insets(20)
      spacing = 10
      style = "-fx-border-color: black; -fx-padding: 10;"
    }

    // Card 2: Calculate Quote
    val partCostInput = new TextField {
      promptText = "Part Cost"
    }

    val calculateQuoteCard = new VBox {
      children = Seq(new Label("Calculate Quote"), partCostInput, new Button("Confirm") {
        onAction = (ae: ActionEvent) => {
          try {
            val cost = partCostInput.text.value.toDouble // .value is crucial in ScalaFX
            partCosts.add(cost)
            partCostInput.clear()
            println(s"Added cost: $cost, Total costs: ${partCosts.sum}")
          } catch {
            case e: NumberFormatException =>
              println("Invalid part cost. Please enter a number.")
          }
        }
      })
      padding = Insets(20)
      spacing = 10
      style = "-fx-border-color: black; -fx-padding: 10;"
    }

    val costsListLabel = new Label("Part Costs:")
    val costsListView = new ListView[Double](partCosts)

    val quoteLayout = new VBox {
      children = Seq(knownPartsCard, calculateQuoteCard, costsListLabel, costsListView)
      alignment = Pos.Center
      padding = Insets(20)
      spacing = 20
    }

    new Scene(quoteLayout, 600, 400)
  }

  // Main Scene
  override def start(): Unit = {
    JFXApp3.userAgentStylesheet = getClass.getResource("primer-light.css").toExternalForm
    val quoteButton = new Button("Quote") {
      onAction = _ => stage.scene = quoteScene
    }
    val formsButton = new Button("Forms") // Add your forms logic here later
    val titleLabel = new Label("Welcome to the App")

    val mainLayout = new VBox {
      children = Seq(titleLabel, new HBox {
        children = Seq(quoteButton, formsButton)
        alignment = Pos.Center
        spacing = 20
      })
      alignment = Pos.Center
      padding = Insets(20)
      spacing = 20
    }

    stage = new JFXApp3.PrimaryStage {
      title = "Quote and Forms App"
      scene = new Scene(mainLayout, 400, 200)
    }
  }
}