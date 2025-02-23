import scalafx.scene.control._
import scalafx.scene.layout._
import scalafx.geometry.{Insets, Pos}
import scalafx.collections.ObservableBuffer
import scalafx.beans.property.{ObjectProperty, StringProperty}
import scalafx.Includes._
import scala.math.BigDecimal.RoundingMode
import scalafx.scene.layout.Priority

class Quote extends BaseScene("Quote") {

  // --- Data Model ---
  case class QuoteRow(cost: BigDecimal) {
    def costRounded: String = cost.setScale(2, RoundingMode.UP).toString
  }

  val quoteData = new ObservableBuffer[QuoteRow]()

  def parseAndRound(input: String): Option[BigDecimal] =
    try Some(BigDecimal(input).setScale(2, RoundingMode.UP))
    catch { case _: Exception => None }

  // --- UI Elements ---
  val totalValueLabel = new Label("0.00")
  val depositValueLabel = new Label("0.00")

  // --- Auto-Update Total and Deposit ---
  def updateTotalAndDeposit(): Unit = {
    val total = quoteData
      .foldLeft(BigDecimal(0))((acc, row) => acc + row.cost)
      .setScale(2, RoundingMode.UP)
    totalValueLabel.text = total.toString
    depositValueLabel.text = (total / 2).setScale(2, RoundingMode.UP).toString
  }

  quoteData.onChange((_, _) => updateTotalAndDeposit())

  // --- Named Pane ---
  def namedPane(title: String, content: scalafx.scene.Node): VBox =
    new VBox {
      spacing = 5
      padding = Insets(10)
      children = Seq(
        new Label(title) {
          style = "-fx-font-weight: bold; -fx-font-size: 14px;"
          alignment = Pos.Center
        },
        content
      )
      style =
        "-fx-border-color: gray; -fx-padding: 10; -fx-background-color: #f9f9f9;"
      maxWidth = Double.MaxValue
      alignment = Pos.Center
    }

  // --- Input Forms ---
  def buildQuotePane(labelText: String, onAdd: BigDecimal => Unit): VBox = {
    val input = new TextField { promptText = "Enter cost" }
    val addButton = new Button("Add") {
      onAction = _ => {
        parseAndRound(input.text.value) match {
          case Some(value) =>
            onAdd(value)
            input.clear()
          case None => println("Invalid input")
        }
      }
    }

    val centeredAddButton = new HBox(addButton) {
      alignment = Pos.Center
      padding = Insets(5)
    }

    new VBox(5, new Label(labelText), input, centeredAddButton) {
      padding = Insets(10)
      maxWidth = Double.MaxValue
      alignment = Pos.Center
    }
  }

  val calcQuotePane =
    buildQuotePane("Quotation rule will be applied", value => quoteData += QuoteRow(value))
  val directQuotePane =
    buildQuotePane("Manually add a quote", value => quoteData += QuoteRow(value))

  val leftVBox = new VBox(
    10,
    namedPane("Calculate Quote", calcQuotePane),
    namedPane("Direct Quote", directQuotePane)
  ) {
    padding = Insets(10)
    maxWidth = Double.MaxValue
    alignment = Pos.TopCenter
  }

  // --- Table and Totals ---
  val costColumn = new TableColumn[QuoteRow, String]("Cost") {
    cellValueFactory = { cellData =>
      new StringProperty(cellData.value, "cost", cellData.value.costRounded)
    }
    prefWidth = 150
  }

  val deleteColumn = new TableColumn[QuoteRow, QuoteRow]("Action") {
    cellValueFactory = { cellData =>
      new ObjectProperty[QuoteRow](this, "cell", cellData.value)
    }
    prefWidth = 100
  }

  val deleteCellFactory
      : TableColumn[QuoteRow, QuoteRow] => TableCell[QuoteRow, QuoteRow] =
    (col: TableColumn[QuoteRow, QuoteRow]) =>
      new TableCell[QuoteRow, QuoteRow] {
        val deleteButton = new Button("Delete") {
          onAction = _ => {
            if (item.value != null) quoteData -= item.value
          }
        }
        item.onChange { (_, _, newValue) =>
          graphic = if (newValue == null) null else deleteButton
        }
      }
  deleteColumn.cellFactory = deleteCellFactory

  val tableView = new TableView[QuoteRow](quoteData) {
    columns.setAll(costColumn, deleteColumn) // Ensure only 2 columns exist
    // WARNING CODE SMELL: CALLING JAVAFX
    columnResizePolicy =
      javafx.scene.control.TableView.CONSTRAINED_RESIZE_POLICY_FLEX_LAST_COLUMN
    maxWidth = Double.MaxValue
    minHeight = 150
  }

  val scrollPane = new ScrollPane {
    content = tableView
    fitToWidth = true
    fitToHeight = true
    maxWidth = Double.MaxValue
  }

  val totalHBox = new HBox(5, new Label("Total: "), totalValueLabel) {
    padding = Insets(10)
    maxWidth = Double.MaxValue
    alignment = Pos.Center
  }

  val depositHBox = new HBox(5, new Label("Deposit: "), depositValueLabel) {
    padding = Insets(10)
    maxWidth = Double.MaxValue
    alignment = Pos.Center
  }

  val rightVBox = new VBox(10, scrollPane, totalHBox, depositHBox) {
    padding = Insets(10)
    maxWidth = Double.MaxValue
    alignment = Pos.TopCenter
  }

  // --- Main Layout ---
  val mainHBox = new HBox(10, leftVBox, rightVBox) {
    padding = Insets(10)
    maxWidth = Double.MaxValue
    alignment = Pos.TopCenter
  }

  HBox.setHgrow(leftVBox, Priority.Always)
  HBox.setHgrow(rightVBox, Priority.Always)

  // Populate the contentBox.children from BaseScene.
  contentBox.children = Seq(mainHBox)
}
