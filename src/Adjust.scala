import scalafx.scene.layout.VBox
import scalafx.scene.control.{Label, TextField}
import scalafx.beans.binding.Bindings
import scalafx.geometry.Insets

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
      
      if (difference > 0) s"You are missing $$ $adjustedDifference sales from the EFTPOS machine"
      else if (difference < 0) s"You have recorded $$ ${adjustedDifference.abs} extra sales in Mybug"
      else "No discrepancy detected"
    }, 
    eftposTotalField.text, machineTotalField.text
  )

  // Create labeled containers
  val eftposBox = new VBox(5, new Label("EFTPOS total in Mybug"), eftposTotalField)
  val machineBox = new VBox(5, new Label("Total in the EFTPOS machine"), machineTotalField)

  // Arrange components
  contentBox.children = Seq(
    eftposBox,
    machineBox,
    differenceLabel
  )
  contentBox.spacing = 10
  contentBox.padding = Insets(10)
