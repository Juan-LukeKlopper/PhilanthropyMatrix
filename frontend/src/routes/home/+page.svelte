<script>
  // Check if user is authenticated
  let token = localStorage.getItem('token');
  if (!token) {
    window.location.href = '/login';
  }
</script>

<h1>Welcome to the Home Page</h1>
<p>This is a protected page.</p>

