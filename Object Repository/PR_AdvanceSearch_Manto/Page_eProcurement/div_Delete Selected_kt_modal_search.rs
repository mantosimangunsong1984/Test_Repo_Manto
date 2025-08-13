<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Delete Selected_kt_modal_search</name>
   <tag></tag>
   <elementGuidId>c5b6c9bf-1402-4c83-bbdc-ff0efc24a3dd</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#kt_modal_search</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='kt_modal_search']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#kt_modal_search</value>
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
      <webElementGuid>5a9389dd-49fa-43b0-99ff-06bd743664cb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>modal fade show</value>
      <webElementGuid>dba11ab4-bc17-4533-bb57-872006d2f4a5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>kt_modal_search</value>
      <webElementGuid>141dd00d-8c94-4aa7-aa5d-22c4187d2383</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-bs-backdrop</name>
      <type>Main</type>
      <value>static</value>
      <webElementGuid>237fabdf-5dc4-415e-a592-88041e599ae6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-bs-keyboard</name>
      <type>Main</type>
      <value>false</value>
      <webElementGuid>ba8f3d20-84c9-4c0b-aabf-0b183da6c8fa</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>aria-labelledby</name>
      <type>Main</type>
      <value>staticBackdropLabel</value>
      <webElementGuid>34f1a3c5-93dc-44a5-be9b-adf02f3ae71f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>aria-modal</name>
      <type>Main</type>
      <value>true</value>
      <webElementGuid>6b0e238b-97fd-4a46-af1a-e13404b9cc19</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>role</name>
      <type>Main</type>
      <value>dialog</value>
      <webElementGuid>2d69381c-f36f-4b32-bc3c-a2db85d9f40f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on('apply.daterangepicker', function(ev, picker) {
					$(this).val(picker.startDate.format('YYYY-MM-DD') + ' - ' + picker.endDate.format('YYYY-MM-DD'));
				});

				$(&quot;#created_at&quot;).on('cancel.daterangepicker', function(ev, picker) {
					$(this).val('');
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on('apply.daterangepicker', function(ev, picker) {
					$(this).val(picker.startDate.format('YYYY-MM-DD') + ' - ' + picker.endDate.format('YYYY-MM-DD'));
				});

				$(&quot;#updated_at&quot;).on('cancel.daterangepicker', function(ev, picker) {
					$(this).val('');
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$('form').on('focus', 'input[type=number]', function (e) {
		$(this).on('wheel.disableScroll', function (e) {
			e.preventDefault()
		})
	})

	
	$('input[type=file]').on('change', function(e) {
		var text = $(this).parent().find('.custom-file-label').html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find('.custom-file-label').html(text);
	});

	$('input[type=date]').on('change', function(e) {
		$(this).removeClass('is-invalid');
		$(this).parent().parent().find('span.text-strong.text-danger').remove();
	});

	$('textarea').on('input', function() {
		$(this).removeClass('is-invalid');
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('input').on('input', function() {
		$(this).removeClass('is-invalid');
		if($(this).hasClass('datetimepicker-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else if($(this).hasClass('form-check-input')){
			$(this).parent().parent().find('span.text-strong.text-danger').remove();
		}else{
			$(this).parent().find('span.text-strong.text-danger').remove();
		}
	});

	$('.select2').on('change', function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find('span.text-strong.text-danger').remove();
	});

	$('[data-control=&quot;select2&quot;]').on('change', function() {
		$('#' + $(this).attr('id') + ' + span > span > span').removeClass('is-invalid');
        $(this).parent().find('span.text-strong.text-danger').remove();
	});
});

$('input:radio[name=&quot;is_top_parent&quot;]').change(function() {
	if ($(this).val() == '1') {
		$('#job_title_select').select2('destroy');
		$('#job_title_select').val(&quot;&quot;);
		$('#job_title_select').select2();
		$('#job_title_select').attr('disabled', '');
		$('.job_title_select').hide();

		$('#company_of_parent').select2('destroy');
		$('#company_of_parent').val(&quot;&quot;);
		$('#company_of_parent').select2();
		$('#company_of_parent').attr('disabled', '');
		$('.company_of_parent').hide();

	} else {
		$('#job_title_select').removeAttr('disabled');
		$('.job_title_select').show();

		$('#company_of_parent').removeAttr('disabled');
		$('.company_of_parent').show();
	}
});
$('input[name=&quot;level&quot;]').change(function() {
	if ($(this).val().includes('-')) {
		$(this).val('0');
	}
})

$('input:text[name=&quot;postal_code&quot;]').change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val('');
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$('#kt_modal_search_form #date-now').remove();
															$('#kt_modal_search_form [name=&quot;_token&quot;]').remove();
															$('[data-control=&quot;select2&quot;]').select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			</value>
      <webElementGuid>31fc8e64-c2e5-4014-bff4-acf8b8db1ce7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;kt_modal_search&quot;)</value>
      <webElementGuid>892b8b71-e0af-4bdd-9c5a-19870b40b54c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='kt_modal_search']</value>
      <webElementGuid>a842a4b5-3852-4cb9-a60b-ad3a7150bf69</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='kt_content_container']/div/div/div[2]/div[3]</value>
      <webElementGuid>0e6afbd9-07e9-445c-8aca-b213fce99980</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Delete Selected'])[1]/following::div[1]</value>
      <webElementGuid>0c0a4d0d-d242-49d2-a84a-6cc1726a52c9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div/div/div[2]/div[3]</value>
      <webElementGuid>3b47014e-5d90-4a15-b33d-c2605fd01860</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'kt_modal_search' and (text() = concat(&quot;
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			&quot;) or . = concat(&quot;
				
				
					
					
						
						
							
							Advance Search
							
							
							
								
								
									
										
											
											
										
									
								
								
							
							
						
						
												
						
							
							
								
                                    
                                        
                                            
																																					
													
														
														
														
														
																													
																												
					
			
				
				
					UR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PR Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Title
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Remarks
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Status
					
				
				
				
				-Select Status-DraftOpenRejectedRevisedFully ApprovedCancelledClosedGood IssuedSelect Status...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Total PR
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Purchasing Group
					
				
				
				
				-Select Purchasing Group-200 - Non-Project201 - ProjectSelect Purchasing Group...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Creator
					
				
				
				
				-Select Creator-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Creator...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Requestor
					
				
				
				
				-Select Requestor-AdministratorAkhmad Maulana HamzahYogi Octarendi KartonoKristi LauwMGS.M.Rifat MishbahuddinNita Arsita DewiAde WahyuWilliam GanisRustam Sofyan SiraitWirata Adhidharma WidajaSandy SusantoWillyWardaniVirna Firyal MuslimJenina Fricilia SinagaMartin HauwitaGiatrycks Freddy SianiparJoseph Arifianto SoenarjoKurniawan IrwantoIqbal CaesariawanNovia Mathilda FransiscaTedy HarjantoEfa SusenoStephani Puspitajati HHelen KusumawatiRenata HartadiYuniatiHikmat SuryahimandaMuhammad Nur RidwanEsther Suzanna PakpahanFransisca Duma SiagianNovian RahmatAninda Ayu ListiaMichaelAnastasia Anne AugustaChristopher T WatulingasEric Joses SulaimanKhasan BashriJuli ParlindunganLinda RahmadhaniOlief OngkowidjojoSitor Suwardi SFajar SiddikNova Indra PratamaRiduan Marcellino BancinIndri Nur AiniDhiyaul HudaJuliviandini Trienda PutriMaria Estevani Rotua StyawatiAmbarjantiWahyu Indra LesmanaAmanda Oktina DahlerAryadi Kristianto SimanjuntakFarah SalsabilaIkhwan SuhaidiInsan KarunaReza Pahlevi Alexander TaopanNovaldy Antonio WundNovy NugrohoReza MaulanaGayatri Hemakumar VijayakumariAsep SaepudinRio AriandiFirman SyahlaniImam DwicaksonoBrilian Ade PutraIntan FathimahSurya KencanaKresna Ivan NugrahaMuhammad AdnanAnnisa Citra IslamiatiFloorian RegieNuraini Soviany PutriDina Intan SariSekar Amelia FujiantySarah Ningtyas RachmanOlivia Yulianne AugiSharone SabbathaniaHafizh ZonaKrisnovri HartatiPutri Dwi NantiAndi Alifesa AskariAryaRyan Maulana HarunDaniel Fernando HsHandoko SaidEstiko SaputroSylvia VeronicaJusiera Zahra SyawaliMeydina AndrianiCitra Y AritonangLaksmi IndrawatiAde Nurmarita SariMeliana HusenMohammad Rafel NizamBenny HalimFebdroEllyva Shintu AngeliaSendy GretiApi Login Prisca IvantiHalley Miraj DanarGuruh GunawanStephanie Lydia Apriliana SilitoSHERINA ARTHARIANI ZUKRIANTOBryan VM. SianiparSatrio WibisonoMaria Angelina YauwtantyWelly RenatoMichelle WidiapranoloDaniel TonywanNovi YantiAde DanuArmando Bonisius NottyHafizh JafriSilvano Winston RumantirIntan EmeraldaAilinawati BudimanEldin Muhamad AkbarFransiskus Agus Permana PutraAMS System API UserRiza Diah WidyaningsihMaulida Disti SukandarYayah KhoriyahRizki AmaliaMochamad Ieqbal Hario PrakasaOkky Mahdi YasserInayah Pili AndiniLaura Puspita SariZidan HamdaniAgatha Citra Vannesa DrupadiDina LestariAndhika Rama SantosoTeddy YusupRobinsyah SibueaMarshal Andreas SibaraniIgnasius Yakob MeringMelindaIndira kartika SariYolanda NainggolanRosliane Stefani HutapeaYussy SantosoLaksana IrwantoAtika Fiza AqilaSuci Yanti DewiOndy Asep SaputraBagas Praditya Yusuf PutraAnnisa Permata DewiMuhammad FikriMukhamad NurrokhmanAngga Kurnia PutraAli MusodikRudy HermawanMargareta Deke Sukmawati N.LDedi KusnandarNiken TjandraSteffani Candra WijayaJessica FlorenciaFredy LimanauwMuhamad Hakim RafsanjaniKuncara Putra UtamaApriyanti HidayatEndy UtomoViona Rosalin PriscillaMantoSelect Requestor...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					Department
					
				
				
				
				-Select Department-SUSTAINABILITY &amp; PROJECTSUSTAINABILITYSUSTAINABILITY &amp; PROJECT - PROJECTCORPORATE RELATIONTREASURY &amp; BANKING RELATIONTREASURY &amp; BANKING RELATION - DEPTFINANCE, TAX &amp; ACCOUNTINGFINANCE OPERATION ACCOUNTING &amp; TAXFINANCE STRATEGIC REPORTING &amp; BUDGET CONTROLINTERNAL AUDIT &amp; RISK MANAGEMENTINTERNAL AUDITRISK MANAGEMENTSTRATEGIC &amp; BUSINESS PROCESS MANAGEMENTSTRATEGIC MANAGEMENT OFFICEBUSINESS PROCESS, IMPROVEMENT &amp; AUTOMATIONLAND &amp; ASSETSCOMPLIANCELEGAL BUSINESS &amp; FINANCINGINFORMATION TECHNOLOGYIT INFRASTRUCTURESERVICE MANAGEMENT &amp; SUPPORTIT BUSINESS PARTNER &amp; BUSINESS ANALYSTIT DEVELOPMENTGENERAL AFFAIRHUMAN RESOURCES &amp; GENERAL AFFAIRHR CENTRE OF EXCELLENCEHR COE - OD &amp; PMHR COE - DATA ANALYTICSHR SHARED SERVICEHR SS - PAYROLL &amp; BENEFITHR SS - HRISEXECUTIVE ASSISTANTHR SS - L&amp;DPERSONAL ASSISTANTBUSINESS DEVELOPMENT HOTREASURY &amp; BANKING PERSONAL ASSISTANTCORPORATE COMMUNICATION &amp; EVENTCORPORATE COMMUNICATIONMEDIA RELATIONASSET MANAGEMENT &amp; PROCUREMENTASSIGNMENTFounder OfficeHR - Organization DevelopmentHR - HRISHR - Reward &amp; Employee ServiceHR - Talent ManagementIT SECURITY AND COMPLIANCESelect Department...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Cost Center
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					On Bidding
					
				
				
				
				-Select On Bidding-YesNoSelect On Bidding...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					Bidding Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
								
			
				
				
					PO Fully Created
					
				
				
				
				-Select PO Fully Created-YesNoSelect PO Fully Created...
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
				
					PO Number
					
				
				
				
					
				
							
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Created At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#created_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#created_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																						
													
														
														
														
														
																													
																												
					
			
				
					Updated At
					
				
				
				
					
					
						
							
							
								
									
										
										
										
									
								
							
							
						
					
					
					
					
					
				
				
							
			

			
			$(document).ready( function () {
				$(&quot;#updated_at&quot;).daterangepicker({
					autoUpdateInput: false,
					drops: &quot;up&quot;
				});
				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;apply.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(picker.startDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; - &quot; , &quot;'&quot; , &quot; + picker.endDate.format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;));
				});

				$(&quot;#updated_at&quot;).on(&quot; , &quot;'&quot; , &quot;cancel.daterangepicker&quot; , &quot;'&quot; , &quot;, function(ev, picker) {
					$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
				});
			});
			
				


$.fn.modal.Constructor.prototype._enforceFocus = function() {};
$(document).ready(function() {
	$(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;input[type=number]&quot; , &quot;'&quot; , &quot;, function (e) {
		$(this).on(&quot; , &quot;'&quot; , &quot;wheel.disableScroll&quot; , &quot;'&quot; , &quot;, function (e) {
			e.preventDefault()
		})
	})

	
	$(&quot; , &quot;'&quot; , &quot;input[type=file]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		var text = $(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html();
		text = text.substring(text.lastIndexOf(&quot;\\&quot;) + 1, text.length);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;.custom-file-label&quot; , &quot;'&quot; , &quot;).html(text);
	});

	$(&quot; , &quot;'&quot; , &quot;input[type=date]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(e) {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;textarea&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;, function() {
		$(this).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
		if($(this).hasClass(&quot; , &quot;'&quot; , &quot;datetimepicker-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else if($(this).hasClass(&quot; , &quot;'&quot; , &quot;form-check-input&quot; , &quot;'&quot; , &quot;)){
			$(this).parent().parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}else{
			$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
		}
	});

	$(&quot; , &quot;'&quot; , &quot;.select2&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot;#&quot; + $(this).attr(&quot;id&quot;) + &quot; + span&quot;).removeClass(&quot;is-invalid&quot;);
		$(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});

	$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
		$(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot; + $(this).attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot; + span > span > span&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-invalid&quot; , &quot;'&quot; , &quot;);
        $(this).parent().find(&quot; , &quot;'&quot; , &quot;span.text-strong.text-danger&quot; , &quot;'&quot; , &quot;).remove();
	});
});

$(&quot; , &quot;'&quot; , &quot;input:radio[name=&quot;is_top_parent&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val() == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).hide();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2(&quot; , &quot;'&quot; , &quot;destroy&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).val(&quot;&quot;);
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).select2();
		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).hide();

	} else {
		$(&quot; , &quot;'&quot; , &quot;#job_title_select&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.job_title_select&quot; , &quot;'&quot; , &quot;).show();

		$(&quot; , &quot;'&quot; , &quot;#company_of_parent&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
		$(&quot; , &quot;'&quot; , &quot;.company_of_parent&quot; , &quot;'&quot; , &quot;).show();
	}
});
$(&quot; , &quot;'&quot; , &quot;input[name=&quot;level&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if ($(this).val().includes(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;)) {
		$(this).val(&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;);
	}
})

$(&quot; , &quot;'&quot; , &quot;input:text[name=&quot;postal_code&quot;]&quot; , &quot;'&quot; , &quot;).change(function() {
	if (isNaN(Number($(this).val()))) {
		alert(&quot;Postal Code Value is not a number&quot;);
		if (window.alert) {
			$(this).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
		}
	} else if ($(this).val().length > 5) {
		alert(&quot;Postal Code Value is at least 5&quot;);
		if (window.alert) {
			$(this).val($(this).val().slice(0, 5));
		}
	}
})


														
														$(document).ready( function () {
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form #date-now&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;#kt_modal_search_form [name=&quot;_token&quot;]&quot; , &quot;'&quot; , &quot;).remove();
															$(&quot; , &quot;'&quot; , &quot;[data-control=&quot;select2&quot;]&quot; , &quot;'&quot; , &quot;).select2({
																allowClear: true
															});
														});
														
														
													
																																				
                                        
                                    
                                
								
								
									Close
									Clear
									
										Search
										Please wait...
										
									
								
								
							
							
						
						
											
					
				
				
			&quot;))]</value>
      <webElementGuid>d12ad696-f69f-4bf6-888a-aa2824e2272e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
