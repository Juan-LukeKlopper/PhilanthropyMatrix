<script>
    import { minidenticon } from 'minidenticons';
  
    export let username = '';
  
    let svgHtml = '';
  
    $: svgHtml = minidenticon(username);
  </script>
  
  <div class="minidenticon-container">
    {@html svgHtml}
  </div>
  
  <style>
    .minidenticon-container {
      border-radius: 50%;
      background-color: grey;
      height: 60px;
      width: 60px;
    }
  </style>
  