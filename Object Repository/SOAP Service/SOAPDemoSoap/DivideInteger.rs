<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DivideInteger</name>
   <tag></tag>
   <elementGuidId>0517ad77-a690-4f79-816c-13bca5124a51</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/SOAP.Demo.DivideInteger</value>
      <webElementGuid>4992bab8-0e7a-4a6c-97e5-4d710f64a154</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>4bbd9ab5-1cf5-4a13-96d3-14a46d34a794</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:DivideInteger>
         &lt;tem:Arg1>${total_sum}&lt;/tem:Arg1>
         &lt;tem:Arg2>${int_1}&lt;/tem:Arg2>
      &lt;/tem:DivideInteger>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>https://www.crcind.com:443/csp/samples/SOAP.Demo.cls</soapServiceEndpoint>
   <soapServiceFunction>DivideInteger</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.SOAP_Total</defaultValue>
      <description></description>
      <id>e96fe780-bb34-40d3-b76e-1e2b380920c4</id>
      <masked>false</masked>
      <name>total_sum</name>
   </variables>
   <variables>
      <defaultValue>'2'</defaultValue>
      <description></description>
      <id>27cb9df9-2300-4a2f-8a8e-42535c091b89</id>
      <masked>false</masked>
      <name>int_1</name>
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

Result =WS.getElementPropertyValue(response, 'DivideIntegerResponse.DivideIntegerResult')


def variable1 = request.getVariables()
def var1 = variable1.get('int_1')
def var2 = variable1.get('total_sum')

System.out.println(var2)
System.out.println(var1)
System.out.println(Result)</verificationScript>
   <wsdlAddress>https://www.crcind.com/csp/samples/SOAP.Demo.CLS?WSDL=1</wsdlAddress>
</WebServiceRequestEntity>
