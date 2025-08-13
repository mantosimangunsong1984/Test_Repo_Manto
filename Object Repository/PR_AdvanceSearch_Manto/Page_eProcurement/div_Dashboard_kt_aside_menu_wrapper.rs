<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Dashboard_kt_aside_menu_wrapper</name>
   <tag></tag>
   <elementGuidId>8d716121-18cc-4b82-a9c3-e97887da464c</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#kt_aside_menu_wrapper</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='kt_aside_menu_wrapper']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#kt_aside_menu_wrapper</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>43758747-bdb7-4500-ae13-879a5f9e0e6e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>hover-scroll-overlay-y my-5 my-lg-5</value>
      <webElementGuid>67699a4f-9473-4c07-b446-788e5de99375</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>kt_aside_menu_wrapper</value>
      <webElementGuid>9644ab22-653f-4735-a3a4-0dc67cd70b9f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll</name>
      <type>Main</type>
      <value>true</value>
      <webElementGuid>1d1f5b48-87c2-4e7f-aff1-de311e288fa6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll-activate</name>
      <type>Main</type>
      <value>{default: false, lg: true}</value>
      <webElementGuid>9cb90bf8-eda4-42c0-8399-21dd70b0d99b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll-height</name>
      <type>Main</type>
      <value>auto</value>
      <webElementGuid>5dab2269-dd29-4947-945e-9aabd946c641</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll-dependencies</name>
      <type>Main</type>
      <value>#kt_aside_logo, #kt_aside_footer</value>
      <webElementGuid>a3c94a84-25c5-4a43-ba35-1f8dcec0d4b8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll-wrappers</name>
      <type>Main</type>
      <value>#kt_aside_menu</value>
      <webElementGuid>4d7e26fb-a6bb-4eda-aaa0-59da7dec4c06</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-kt-scroll-offset</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>b4d03ea9-34b4-4108-9b01-6eb52bdacc46</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
			
			
			
					
				
					
						
						
							
						
						
					
					Dashboard
				
			
					
					
					
					
				
					
						
						
							
						
						
					
					Purchase Request
				
			
					
					
					
					
					
				
					
						
						
							
						
						
					
					Tender Process
				
			
					
					
					
					
					
					
				
					
						
						
							
						
						
					
					Invoice
				
			
					
					
				
					
						
						
							
						
						
					
					Vendor Evaluation
				
			
					
					
					
					
			


	$(document).ready( function () {
		$(&quot;.menu-link&quot;).on(&quot;click&quot;, function(){
			var href = $(this).attr('href');
			var split_href = href.split('/');
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem('DataTables_kt_datatable_example_1_/'+module+'/'+action)
		});

		if (performance.navigation.type == performance.navigation.TYPE_RELOAD) 
		{
			var href = window.location.href;
			var split_href = href.split('/');
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem('DataTables_kt_datatable_example_1_/'+module+'/'+action)

			// $('#clear_advance_search').click();
		}
	});

			
		</value>
      <webElementGuid>9bdd06a4-06bc-4ab5-9d3c-8d0e9a8bb24e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;kt_aside_menu_wrapper&quot;)</value>
      <webElementGuid>78b457cd-af1c-438b-a7a0-8afb9a863326</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='kt_aside_menu_wrapper']</value>
      <webElementGuid>547dfb63-3d80-4811-aecd-548e0d8dba5f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='kt_aside']/div[2]/div</value>
      <webElementGuid>0d09d07c-7474-4972-9e07-085bc2941003</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div[2]/div</value>
      <webElementGuid>4ef4a0a8-1645-4ece-8a7f-e4f9bc305957</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'kt_aside_menu_wrapper' and (text() = concat(&quot;
			
			
			
					
				
					
						
						
							
						
						
					
					Dashboard
				
			
					
					
					
					
				
					
						
						
							
						
						
					
					Purchase Request
				
			
					
					
					
					
					
				
					
						
						
							
						
						
					
					Tender Process
				
			
					
					
					
					
					
					
				
					
						
						
							
						
						
					
					Invoice
				
			
					
					
				
					
						
						
							
						
						
					
					Vendor Evaluation
				
			
					
					
					
					
			


	$(document).ready( function () {
		$(&quot;.menu-link&quot;).on(&quot;click&quot;, function(){
			var href = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
			var split_href = href.split(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;);
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem(&quot; , &quot;'&quot; , &quot;DataTables_kt_datatable_example_1_/&quot; , &quot;'&quot; , &quot;+module+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+action)
		});

		if (performance.navigation.type == performance.navigation.TYPE_RELOAD) 
		{
			var href = window.location.href;
			var split_href = href.split(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;);
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem(&quot; , &quot;'&quot; , &quot;DataTables_kt_datatable_example_1_/&quot; , &quot;'&quot; , &quot;+module+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+action)

			// $(&quot; , &quot;'&quot; , &quot;#clear_advance_search&quot; , &quot;'&quot; , &quot;).click();
		}
	});

			
		&quot;) or . = concat(&quot;
			
			
			
					
				
					
						
						
							
						
						
					
					Dashboard
				
			
					
					
					
					
				
					
						
						
							
						
						
					
					Purchase Request
				
			
					
					
					
					
					
				
					
						
						
							
						
						
					
					Tender Process
				
			
					
					
					
					
					
					
				
					
						
						
							
						
						
					
					Invoice
				
			
					
					
				
					
						
						
							
						
						
					
					Vendor Evaluation
				
			
					
					
					
					
			


	$(document).ready( function () {
		$(&quot;.menu-link&quot;).on(&quot;click&quot;, function(){
			var href = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
			var split_href = href.split(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;);
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem(&quot; , &quot;'&quot; , &quot;DataTables_kt_datatable_example_1_/&quot; , &quot;'&quot; , &quot;+module+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+action)
		});

		if (performance.navigation.type == performance.navigation.TYPE_RELOAD) 
		{
			var href = window.location.href;
			var split_href = href.split(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;);
			var module = split_href[split_href.length - 2]
			var action = split_href[split_href.length - 1]
			localStorage.removeItem(&quot; , &quot;'&quot; , &quot;DataTables_kt_datatable_example_1_/&quot; , &quot;'&quot; , &quot;+module+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+action)

			// $(&quot; , &quot;'&quot; , &quot;#clear_advance_search&quot; , &quot;'&quot; , &quot;).click();
		}
	});

			
		&quot;))]</value>
      <webElementGuid>7cef4c50-4afb-4c3b-9261-b51c15f5c706</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
