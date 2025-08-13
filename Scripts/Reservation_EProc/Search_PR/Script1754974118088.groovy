import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Login_EProc/Login_EProc-Positive'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/span_Dashboard_menu-title'))

WebUI.click(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/a_Sign Out_kt_button_search'))

WebUI.setText(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_UR Number_ur_number'), 
    'test123')

WebUI.setText(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_PR Number_pr_number'), 
    '1234567')

WebUI.setText(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_Title_title'), 
    'Testing')

WebUI.setText(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_Remarks_remarks'), 
    'testing123')

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_Remarks_remarks'), 
    Keys.chord(Keys.TAB))

WebUI.sendKeys(findTestObject('Select Status/Page_eProcurement/span_Status_select2-selection select2-selec_c3fe6d'), Keys.chord(
        Keys.ENTER))

WebUI.setText(findTestObject('Select Status/Page_eProcurement/input_Apply_select2-search__field'), 'Open')

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/span_Status_select2-selection select2-selec_c3fe6d'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_Total PR_total_pr'), 
    '999')

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/input_Total PR_total_pr'), 
    Keys.chord(Keys.TAB))

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/span_Purchasing Group_select2-selection sel_31f095'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Search_PrGroup/Page_eProcurement/input__select2-search__field'), '200 - Non-Project')

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/span_Purchasing Group_select2-selection sel_31f095'), 
    Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/Search_PR/Page_eProcurement/span_Purchasing Group_select2-selection sel_31f095'), 
    Keys.chord(Keys.TAB))

WebUI.sendKeys(findTestObject('Search_PrGroup/Page_eProcurement/span_Creator_select2-selection select2-sele_c469d0'), Keys.chord(
        Keys.ENTER))

WebUI.setText(findTestObject('Search_PrGroup/Page_eProcurement/input_Apply_select2-search__field'), 'Administrator')

WebUI.sendKeys(findTestObject('Search_PrGroup/Page_eProcurement/span_Creator_select2-selection select2-sele_c469d0'), Keys.chord(
        Keys.ENTER))

WebUI.sendKeys(findTestObject('Search_PrGroup/Page_eProcurement/span_Creator_select2-selection select2-sele_c469d0'), Keys.chord(
        Keys.TAB))

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_Requestor_select2-selection select2-selection--single col-md-6 form-select form-select-solid'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Apply_select2-search__field'), 'Administrator')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_Requestor_select2-selection select2-selection--single col-md-6 form-select form-select-solid'), 
    Keys.chord(Keys.ENTER))

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_Requestor_select2-selection select2-selection--single col-md-6 form-select form-select-solid'), 
    Keys.chord(Keys.TAB))

WebUI.sendKeys(findTestObject('Dropdown Departemen/Page_eProcurement/span_Department_select2-selection select2-s_7ed897'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Dropdown Departemen/Page_eProcurement/input__select2-search__field'), 'IT BUSINESS PARTNER')

WebUI.sendKeys(findTestObject('Dropdown Departemen/Page_eProcurement/span_Department_select2-selection select2-s_7ed897'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Cost Center_cost_center'), 'B1000')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/input_Cost Center_cost_center'), Keys.chord(Keys.TAB))

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_On Bidding_select2-selection select2-s_b4720b'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Apply_select2-search__field'), 'NO')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_On Bidding_select2-selection select2-s_b4720b'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Bidding Number_bidding_number'), '00000')

WebUI.sendKeys(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/input_Bidding Number_bidding_number'), Keys.chord(
        Keys.TAB))

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_PO Fully Created_select2-selection sel_0c8ad8'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Apply_select2-search__field'), 'NO')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/span_PO Fully Created_select2-selection sel_0c8ad8'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_PO Number_po_number'), 'ABC123')

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Created At_created_at'), '2025-08-06 - 2025-08-31')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/input_Created At_created_at'), Keys.chord(Keys.TAB))

WebUI.setText(findTestObject('Advance Search_2/Page_eProcurement/input_Updated At_updated_at'), '2025-09-01 - 2025-09-15')

WebUI.sendKeys(findTestObject('Advance Search_2/Page_eProcurement/input_Updated At_updated_at'), Keys.chord(Keys.TAB))

WebUI.click(findTestObject('PR_AdvanceSearch_Manto/Page_eProcurement/button_Clear_btn btn-primary'))

WebUI.delay(5)

WebUI.takeFullPageScreenshot()

