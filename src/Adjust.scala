import scalafx.scene.layout.{VBox, HBox}
import scalafx.scene.control.{Label, TextField}
import scalafx.beans.binding.Bindings
import scalafx.geometry.{Insets, Pos}

class Adjust extends BaseScene("Adjust POS"):
  // Create input fields for the two values
  val eftposTotalField = new TextField {
    promptText = "Enter EFTPOS total"
  }

  val machineTotalField = new TextField {
    promptText = "Enter machine total"
  }

  // Label to display the difference
  val differenceLabel = new Label("Difference: 0.00")

  // Helper function to safely parse input to BigDecimal
  private def parseBigDecimal(input: String): BigDecimal =
    try BigDecimal(input) catch case _: NumberFormatException => BigDecimal(0)

  // Bind the label to the difference between the two fields
  differenceLabel.text <== Bindings.createStringBinding(
    () => {
      val eftposTotal = parseBigDecimal(eftposTotalField.text.value.trim)
      val machineTotal = parseBigDecimal(machineTotalField.text.value.trim)

      val difference = (eftposTotal - machineTotal).setScale(2, BigDecimal.RoundingMode.HALF_UP)
      val adjustedDifference = (difference - (difference * 0.009)).setScale(2, BigDecimal.RoundingMode.HALF_UP)

      if (difference > 0) s"You are missing $$ ${adjustedDifference.formatted("%.2f")} sales from the EFTPOS machine"
      else if (difference < 0) s"You have recorded $$ ${adjustedDifference.abs.formatted("%.2f")} extra sales in Mybug"
      else "No discrepancy detected"
    },
    eftposTotalField.text, machineTotalField.text
  )

  // Create labeled containers with larger horizontal size and centered labels
  val eftposBox = new VBox(5, new Label("EFTPOS total in Mybug") {
    style = "-fx-font-size: 16px; -fx-font-weight: bold;"
  }, eftposTotalField) {
    // Increase width to 250px and center the content
    prefWidth = 250
    alignment = Pos.Center
  }

  val machineBox = new VBox(5, new Label("Total in the EFTPOS machine") {
    style = "-fx-font-size: 16px; -fx-font-weight: bold;"
  }, machineTotalField) {
    // Increase width to 250px and center the content
    prefWidth = 250
    alignment = Pos.Center
  }

  // Wrap input fields inside an HBox and center it
  val inputContainer = new HBox(20, eftposBox, machineBox) {
    padding = Insets(20)
    alignment = Pos.Center
  }

  // Style differenceLabel to embolden the difference text
  differenceLabel.style = "-fx-font-size: 14px; -fx-font-weight: normal;" // Default font weight

  // Arrange components and center the inputContainer at the top
  contentBox.children = Seq(
    inputContainer,
    differenceLabel
  )
  contentBox.spacing = 15
  contentBox.padding = Insets(20)
  contentBox.alignment = Pos.TopCenter // Ensures input container is at the top-center
