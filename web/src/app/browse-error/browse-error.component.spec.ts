import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BrowseErrorComponent } from './browse-error.component';

describe('BrowseErrorComponent', () => {
  let component: BrowseErrorComponent;
  let fixture: ComponentFixture<BrowseErrorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ BrowseErrorComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(BrowseErrorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
