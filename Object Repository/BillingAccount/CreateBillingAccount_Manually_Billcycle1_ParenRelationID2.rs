<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateBillingAccount_Manually_Billcycle1_ParenRelationID2</name>
   <tag></tag>
   <elementGuidId>76614d50-8a95-4334-b9de-b3fff2bc7a8d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot; standalone=&quot;no&quot;?>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:aom=&quot;http://www.i2i.com/fcbs/soa/schemas/AOMWS&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;aom:CreateBillingAccount>
         &lt;pReqInfo>
            &lt;ACTION_INTERACTION_TYPE_ID>143&lt;/ACTION_INTERACTION_TYPE_ID>
            &lt;ACTION_DATE>2019/02/01-08:01:00&lt;/ACTION_DATE>
            &lt;TRANSACTION_ID>CA147852&lt;/TRANSACTION_ID>
            &lt;CUSTOMER_ID>${custID}&lt;/CUSTOMER_ID>
            &lt;USER_NAME>cansuarslan&lt;/USER_NAME>
            &lt;CHANNEL_NAME>cansuarslan&lt;/CHANNEL_NAME>
            &lt;SIMULATION_FLAG>false&lt;/SIMULATION_FLAG>
            &lt;MODIFY_INFO>
               &lt;CREATE_INFO>
                  &lt;CREATE_DATE>2019/02/01-08:01:00&lt;/CREATE_DATE>
                  &lt;CREATE_USER>cansuarslan&lt;/CREATE_USER>
               &lt;/CREATE_INFO>
            &lt;/MODIFY_INFO>
         &lt;/pReqInfo>
         &lt;pBillAcct>
            &lt;BILLING_ACCOUNT_KEY>
               &lt;BILLING_ACCOUNT_ID>${billAcctID}&lt;/BILLING_ACCOUNT_ID>
            &lt;/BILLING_ACCOUNT_KEY>
            &lt;BILLING_ACCOUNT_DEFINITION>
               &lt;NAME_ON_INVOICE>Cansu Arslan&lt;/NAME_ON_INVOICE>
               &lt;PARENT_RELATION_ID>${parentRelationID1}&lt;/PARENT_RELATION_ID>
               &lt;ACCOUNT_TYPE>1&lt;/ACCOUNT_TYPE>
               &lt;ACCOUNT_STATUS>A&lt;/ACCOUNT_STATUS>
               &lt;PIN_CODE>3040&lt;/PIN_CODE>
               &lt;COMPANY_DEFINITION_ID>1&lt;/COMPANY_DEFINITION_ID>
               &lt;LATE_PAYMENT_EXEMPTION_FLAG>N&lt;/LATE_PAYMENT_EXEMPTION_FLAG>
               &lt;ACCOUNT_TAX_INFO>1&lt;/ACCOUNT_TAX_INFO>
            &lt;/BILLING_ACCOUNT_DEFINITION>
            &lt;BILLING_ACCOUNT_PROFILE>
               &lt;BILL_PRESENTATION_ID>2&lt;/BILL_PRESENTATION_ID>
               &lt;CREDIT_LIMIT>1000&lt;/CREDIT_LIMIT>
               &lt;CURRENCY_CODE>TRL&lt;/CURRENCY_CODE>
               &lt;FCBS_BILL_CYCLE_ID>${billcycle}&lt;/FCBS_BILL_CYCLE_ID>
               &lt;CHARGING_TYPE>2&lt;/CHARGING_TYPE>
               &lt;BILL_ACCT_TYPE>1&lt;/BILL_ACCT_TYPE>
            &lt;/BILLING_ACCOUNT_PROFILE>
            &lt;BILLING_ACCOUNT_CONTACT>
               &lt;CONTACT_TYPE>P&lt;/CONTACT_TYPE>
               &lt;CONTACT_ADDRESS>
                  &lt;POSTAL_ADDRESS>
                     &lt;COUNTRY>TURKEY&lt;/COUNTRY>
                     &lt;CITY_NAME>ADANA&lt;/CITY_NAME>
                     &lt;SUB_PROVINCE_NAME>KOZAN&lt;/SUB_PROVINCE_NAME>
                     &lt;DISTRICT>MERKEZ&lt;/DISTRICT>
                     &lt;MAIN_STREET>KARACAOĞLAN&lt;/MAIN_STREET>
                     &lt;STREET>ADANA&lt;/STREET>
                     &lt;BUILDING>ARSLAN&lt;/BUILDING>
                     &lt;HOUSE_NUMBER>59&lt;/HOUSE_NUMBER>
                     &lt;FLOOR_NUMBER>2&lt;/FLOOR_NUMBER>
                     &lt;DOOR_NUMBER>2&lt;/DOOR_NUMBER>
                     &lt;POSTAL_CODE>01510&lt;/POSTAL_CODE>
                     &lt;DESCRIPTION>Hastane yanı Su deposu önü&lt;/DESCRIPTION>
                  &lt;/POSTAL_ADDRESS>
               &lt;/CONTACT_ADDRESS>
            &lt;/BILLING_ACCOUNT_CONTACT>
            &lt;BILLING_ACCOUNT_CHAR_VAL>
               &lt;NAME>cansuarslan&lt;/NAME>
            &lt;/BILLING_ACCOUNT_CHAR_VAL>
         &lt;/pBillAcct>
      &lt;/aom:CreateBillingAccount>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>CreateBillingAccount</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.billcycle</defaultValue>
      <description></description>
      <id>94b20038-efc6-4f57-bff3-a17155c4100d</id>
      <masked>false</masked>
      <name>billcycle</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.custID</defaultValue>
      <description></description>
      <id>242c5fea-d9aa-4000-94a1-385881501684</id>
      <masked>false</masked>
      <name>custID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.billAcctID</defaultValue>
      <description></description>
      <id>98c8b779-27b8-48ab-ac20-c9714fa874ec</id>
      <masked>false</masked>
      <name>billAcctID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.parentRelationID2</defaultValue>
      <description></description>
      <id>e9a4a965-012b-4830-b19f-e47087e2e519</id>
      <masked>false</masked>
      <name>parentRelationID2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))</verificationScript>
   <wsdlAddress>http://172.30.10.32:8181/AOMWSBillingAccount/AOMWSBillingAccount?wsdl</wsdlAddress>
</WebServiceRequestEntity>
