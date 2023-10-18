import { useSelector } from 'react-redux'

export function Header() {
  return (
    <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <div class="navbar-item">
          <span class="fa-stack fa-lg">
            <i class="fa fa-square fa-stack-2x"></i>
            <i class="fa fa-motorcycle fa-stack-1x fa-inverse"></i>
          </span>
          <h1>Estimation Engine</h1>
        </div>

        <Topic />
      </div>
    </nav>
  );
}

function Topic() {
  let id = useSelector((store) => store.id);

  if (id === null) {
    return null;
  }

  return (
    <div class="navbar-item">
      Room Link: &nbsp;
      <span class="is-family-monospace">
        {window.location.href}
      </span>
    </div>
  );
}